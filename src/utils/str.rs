//! Global, permanent, packed, hashconsed, short string storage.
//!
//! * supports strings up to 256 bytes
//! * derefs to a &str, but uses only 1 word on the stack and len + 1 bytes on the heap
//! * the actual bytes are stored packed into 1 MiB allocations to
//!   avoid the overhead of lots of small mallocs
//! * Copy!
//! * hashconsed, the same &str will always produce a pointer to the same memory
//!
//! CAN NEVER BE DEALLOCATED

use anyhow::bail;
use fxhash::FxHashSet;
use once_cell::sync::Lazy;
use parking_lot::Mutex;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::{
    borrow::{Borrow, Cow},
    collections::HashSet,
    fmt,
    hash::Hash,
    mem,
    ops::Deref,
    slice, str,
};

const TAG_MASK: usize = 0x8000_0000_0000_0000;
const LEN_MASK: usize = 0x7F00_0000_0000_0000;
const CHUNK_SIZE: usize = 1024 * 1024;

struct Chunk {
    data: Vec<u8>,
    pos: usize,
}

impl Chunk {
    #[cfg(target_pointer_width = "64")]
    fn new() -> &'static mut Self {
        let res = Box::leak(Box::new(Chunk { data: vec![0; CHUNK_SIZE], pos: 0 }));
        assert!((res as *mut Self as usize) & TAG_MASK == 0);
        res
    }

    fn insert(&mut self, str: &[u8]) -> (*mut Chunk, Str) {
        let mut t = self;
        loop {
            if CHUNK_SIZE - t.pos > str.len() {
                t.data[t.pos] = str.len() as u8;
                t.data[t.pos + 1..t.pos + 1 + str.len()].copy_from_slice(str);
                let res = Str(t.data.as_ptr().wrapping_add(t.pos) as usize);
                t.pos += 1 + str.len();
                break (t, res);
            } else {
                t = Self::new();
            }
        }
    }
}

struct Root {
    all: FxHashSet<Str>,
    root: *mut Chunk,
}

unsafe impl Send for Root {}
unsafe impl Sync for Root {}

static ROOT: Lazy<Mutex<Root>> =
    Lazy::new(|| Mutex::new(Root { all: HashSet::default(), root: Chunk::new() }));

#[allow(dead_code)]
struct StrVisitor;

impl serde::de::Visitor<'_> for StrVisitor {
    type Value = Str;

    fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "expecting a string")
    }

    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Str::try_from(s).map_err(|e| E::custom(e.to_string()))
    }
}

#[allow(dead_code)]
#[derive(JsonSchema)]
struct AsStr(&'static str);

/// This is either an immediate containing the string data, if the
/// length is less than 8, or a pointer into static memory that holds
/// the actual str slice if the data length is greater than 7.
///
/// Either way it is 1 word on the stack. In the case of an immediate
/// the length as well as all the bytes are stored in that word, and
/// there is no allocation on the heap. Otherwise the length, as well
/// as the actual bytes of the string are stored on the heap in a
/// compact allocation along with other strings of this type.
///
/// The maximum length of strings of this type is 255
/// characters. try_from will fail if a larger string is specified.
///
/// In either case Deref should be quite cheap, there is no locking to
/// deref.
///
/// In the case of immediates there is never any locking. Otherwise, a
/// global lock must be taken to hashcons the string and, if it isn't
/// already present, insert it in the packed allocation.
#[derive(Clone, Copy, Deserialize, JsonSchema)]
#[serde(try_from = "Cow<str>")]
#[serde(into = "&str")]
#[repr(transparent)]
#[cfg_attr(feature = "juniper", derive(juniper::GraphQLScalar))]
#[cfg_attr(feature = "juniper", graphql(description = "A String type"))]
pub struct Str(#[schemars(with = "AsStr")] usize);

unsafe impl Send for Str {}
unsafe impl Sync for Str {}

impl Str {
    pub fn as_str(&self) -> &str {
        unsafe {
            if self.0 & TAG_MASK > 0 {
                #[cfg(target_endian = "little")]
                {
                    let len = (self.0 & LEN_MASK) >> 56;
                    let ptr = self as *const Self as *const u8;
                    let slice = slice::from_raw_parts(ptr, len);
                    str::from_utf8_unchecked(slice)
                }
                #[cfg(target_endian = "big")]
                {
                    let len = (self.0 & LEN_MASK) >> 56;
                    let ptr = (self as *const Self as *const u8).wrapping_add(1);
                    let slice = slice::from_raw_parts(ptr, len);
                    str::from_utf8_unchecked(slice)
                }
            } else {
                let t = self.0 as *const u8;
                let len = *t as usize;
                let ptr = t.wrapping_add(1);
                let slice = slice::from_raw_parts(ptr, len);
                str::from_utf8_unchecked(slice)
            }
        }
    }

    /// return a static str ref unless self is an immediate
    pub fn as_static_str(&self) -> Option<&'static str> {
        unsafe {
            if self.0 & TAG_MASK > 0 {
                None
            } else {
                Some(mem::transmute::<&str, &'static str>(self.as_str()))
            }
        }
    }

    /// return true if this Str is an immediate
    pub fn is_immediate(&self) -> bool {
        self.0 & TAG_MASK > 0
    }
}

impl fmt::Debug for Str {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &**self)
    }
}

impl fmt::Display for Str {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &**self)
    }
}

impl Serialize for Str {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl Deref for Str {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl Borrow<str> for Str {
    fn borrow(&self) -> &str {
        self.as_str()
    }
}

impl Borrow<str> for &Str {
    fn borrow(&self) -> &str {
        self.as_str()
    }
}

impl AsRef<str> for Str {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl Hash for Str {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.as_str().hash(state)
    }
}

impl PartialEq for Str {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl PartialEq<&str> for Str {
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == *other
    }
}

impl Eq for Str {}

impl PartialOrd for Str {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.as_str().partial_cmp(other.as_str())
    }
}

impl PartialOrd<&str> for Str {
    fn partial_cmp(&self, other: &&str) -> Option<std::cmp::Ordering> {
        self.as_str().partial_cmp(*other)
    }
}

impl Ord for Str {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.as_str().cmp(other.as_str())
    }
}

impl TryFrom<String> for Str {
    type Error = anyhow::Error;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        s.as_str().try_into()
    }
}

impl TryFrom<&str> for Str {
    type Error = anyhow::Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        unsafe {
            let len = s.len();
            if len > u8::MAX as usize {
                bail!("string is too long")
            } else if len < 8 {
                #[cfg(target_endian = "little")]
                {
                    let s = s.as_bytes();
                    let mut i = 0;
                    let mut res: usize = TAG_MASK;
                    res |= len << 56;
                    while i < len {
                        res |= (s[i] as usize) << (i << 3);
                        i += 1;
                    }
                    Ok(Str(res))
                }
                #[cfg(target_endian = "big")]
                {
                    let s = s.as_bytes();
                    let mut i = 0;
                    let mut res: usize = TAG_MASK;
                    res |= len << 56;
                    while i < len {
                        res |= (s[i] as usize) << (48 - (i << 3));
                        i += 1;
                    }
                    Ok(Str(res))
                }
            } else {
                let mut root = ROOT.lock();
                match root.all.get(s) {
                    Some(t) => Ok(*t),
                    None => {
                        let (r, t) = (*root.root).insert(s.as_bytes());
                        root.root = r;
                        root.all.insert(t);
                        Ok(t)
                    }
                }
            }
        }
    }
}

impl TryFrom<Cow<'_, str>> for Str {
    type Error = anyhow::Error;

    fn try_from(s: Cow<str>) -> Result<Self, Self::Error> {
        match s {
            Cow::Borrowed(s) => Str::try_from(s),
            Cow::Owned(s) => Str::try_from(s.as_str()),
        }
    }
}

#[cfg(feature = "juniper")]
impl Str {
    fn to_output<S: juniper::ScalarValue>(&self) -> juniper::Value<S> {
        juniper::Value::scalar(self.as_str().to_string())
    }

    fn from_input<S>(v: &juniper::InputValue<S>) -> Result<Self, String>
    where
        S: juniper::ScalarValue,
    {
        v.as_string_value()
            .map(Self::try_from)
            .ok_or_else(|| format!("Expected `String`, found: {v}"))?
            .map_err(|e| e.to_string())
    }

    fn parse_token<S>(value: juniper::ScalarToken<'_>) -> juniper::ParseScalarResult<S>
    where
        S: juniper::ScalarValue,
    {
        <String as juniper::ParseScalarValue<S>>::from_str(value)
    }
}

#[cfg(feature = "postgres-types")]
impl postgres_types::ToSql for Str {
    postgres_types::to_sql_checked!();

    fn to_sql(
        &self,
        ty: &postgres_types::Type,
        out: &mut bytes::BytesMut,
    ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>> {
        self.as_str().to_sql(ty, out)
    }

    fn accepts(ty: &postgres_types::Type) -> bool {
        String::accepts(ty)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rand::{rng, Rng};

    fn rand_ascii(size: usize) -> String {
        let mut s = String::new();
        for _ in 0..size {
            s.push(rng().random_range(' '..'~'))
        }
        s
    }

    fn rand_unicode(size: usize) -> String {
        let mut s = String::new();
        for _ in 0..size {
            s.push(rng().random())
        }
        s
    }

    #[test]
    fn immediates() {
        for _ in 0..10000 {
            let len = rng().random_range(0..8);
            let s = rand_ascii(len);
            let t0 = Str::try_from(s.as_str()).unwrap();
            assert_eq!(&*t0, &*s);
            let t1 = Str::try_from(s.as_str()).unwrap();
            assert_eq!(t0.0, t1.0)
        }
    }

    #[test]
    fn mixed() {
        for _ in 0..10000 {
            let len = rng().random_range(0..256);
            let s = rand_ascii(len);
            let t0 = Str::try_from(s.as_str()).unwrap();
            assert_eq!(&*t0, &*s);
            let t1 = Str::try_from(s.as_str()).unwrap();
            assert_eq!(t0.0, t1.0)
        }
    }

    #[test]
    fn unicode() {
        for _ in 0..10000 {
            let s = loop {
                let len = rng().random_range(0..128);
                let s = rand_unicode(len);
                if s.len() < 256 {
                    break s;
                }
            };
            let t0 = Str::try_from(s.as_str()).unwrap();
            assert_eq!(&*t0, &*s);
            let t1 = Str::try_from(s.as_str()).unwrap();
            assert_eq!(t0.0, t1.0)
        }
    }
}
