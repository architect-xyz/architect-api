/* Copyright 2023 Architect Financial Technologies LLC. This is free
 * software released under the GNU Affero Public License version 3. */

//! Query language for markets

use crate::Str;
use anyhow::Result;
use chrono::{DateTime, Utc};
use once_cell::sync::Lazy;
use pest::{
    iterators::Pairs,
    pratt_parser::{Assoc, Op, PrattParser},
    Parser,
};
use pest_derive::Parser;
use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, JsonSchema)]
#[serde(tag = "type", content = "value")]
pub enum Query {
    All,
    Regex(Str),
    Base(Str),
    BaseKind(Str),
    Quote(Str),
    // CR-someday alee: more expressive set query
    Pool(Str),
    Venue(Str),
    Route(Str),
    ExchangeSymbol(Str),
    Underlying(Str),
    Expiration(DateQ),
    And(Vec<Query>),
    Or(Vec<Query>),
    Not(Box<Query>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, JsonSchema)]
#[serde(tag = "type", content = "value")]
pub enum DateQ {
    On(DateTime<Utc>),
    OnOrAfter(DateTime<Utc>),
    OnOrBefore(DateTime<Utc>),
    Between(DateTime<Utc>, DateTime<Utc>),
}

#[derive(Parser)]
#[grammar = "symbology/query.pest"]
struct QueryParser;

static PRATT_PARSER: Lazy<PrattParser<Rule>> = Lazy::new(|| {
    PrattParser::new()
        .op(Op::infix(Rule::and, Assoc::Left))
        .op(Op::infix(Rule::or, Assoc::Left))
        .op(Op::prefix(Rule::not))
});

fn parse_expr(pairs: Pairs<Rule>) -> Query {
    PRATT_PARSER
        .map_primary(|primary| match primary.as_rule() {
            Rule::tag_all => Query::All,
            Rule::variant_stringlike => {
                let mut inner = primary.into_inner();
                let tag = inner.next().unwrap().as_str();
                let quoted = inner.next().unwrap().as_str();
                match tag {
                    "Regex" => Query::Regex(Str::try_from(quoted).unwrap()),
                    "Base" => Query::Base(Str::try_from(quoted).unwrap()),
                    "BaseKind" => Query::BaseKind(Str::try_from(quoted).unwrap()),
                    "Quote" => Query::Quote(Str::try_from(quoted).unwrap()),
                    "Pool" => Query::Pool(Str::try_from(quoted).unwrap()),
                    "Venue" => Query::Venue(Str::try_from(quoted).unwrap()),
                    "Route" => Query::Route(Str::try_from(quoted).unwrap()),
                    "Underlying" => Query::Underlying(Str::try_from(quoted).unwrap()),
                    _ => unreachable!(),
                }
            }
            Rule::expr => parse_expr(primary.into_inner()),
            _ => unreachable!(),
        })
        .map_prefix(|op, rhs| match op.as_rule() {
            Rule::not => Query::Not(Box::new(rhs)),
            _ => unreachable!(),
        })
        .map_infix(|lhs, op, rhs| match op.as_rule() {
            Rule::and => Query::And(vec![lhs, rhs]),
            Rule::or => Query::Or(vec![lhs, rhs]),
            _ => unreachable!(),
        })
        .parse(pairs)
}

impl Query {
    pub async fn parse_file_or_query(filename_or_query: &str) -> Result<Self> {
        if tokio::fs::try_exists(filename_or_query).await? {
            let query = tokio::fs::read_to_string(filename_or_query).await?;
            Self::parse(&query)
        } else {
            Self::parse(&filename_or_query)
        }
    }

    pub fn parse(expr: &str) -> Result<Self> {
        let pairs = QueryParser::parse(Rule::expr, expr)?;
        let parsed = parse_expr(pairs);
        Ok(parsed)
    }
}

// #[cfg(test)]
// mod test {
//     use super::*;
//     use crate::{
//         protocol::{cficode::CfiCode, symbology::ProductClass},
//         symbology::Txn,
//     };
//     use rust_decimal_macros::dec;
//
//     fn initialize_symbology_for_test() -> Result<()> {
//         let mut txn = Txn::begin();
//         txn.add_product(
//             "LINK Crypto",
//             &ProductClass::Coin { token_info: Default::default() },
//             CfiCode::default(),
//             dec!(1),
//         )?;
//         txn.add_product("USD", &ProductClass::Fiat, CfiCode::default(), dec!(1))?;
//         txn.add_product("EUR", &ProductClass::Fiat, CfiCode::default(), dec!(1))?;
//         txn.commit()?;
//         Ok(())
//     }
//
//     // XCR alee: test fails intermittently bc of shared symbology races
//     #[test]
//     fn test_parse() {
//         initialize_symbology_for_test().unwrap();
//         let q = Query::parse(r#"Base("LINK Crypto") && Quote(  "USD")"#).unwrap();
//         let link = Product::get("LINK Crypto").unwrap();
//         let usd = Product::get("USD").unwrap();
//         assert_eq!(q, Query::And(vec![Query::Base(link), Query::Quote(usd)]));
//         let q = Query::parse(r#"All && !Quote("EUR")"#).unwrap();
//         assert_eq!(
//             q,
//             Query::And(vec![
//                 Query::All,
//                 Query::Not(Box::new(Query::Quote(Product::get("EUR").unwrap())))
//             ])
//         );
//     }
// }
