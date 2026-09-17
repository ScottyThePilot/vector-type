# `vector-type`

This library provides a `vector_type!` macro which can define a 'Vector Struct' and a 'Vector Enum'.

The vector struct is essentially a generic array type,
where all fields are the same type. Each field corresponds to a
variant in the vector enum, and the vector enum can be used to index
fields of the vector struct.

The bulk of the utility you will get from this macro are the functions, constants,
and traits automatically implemented for the vector struct and vector enum.
See the 'Implementation' section for a list of items that are implemented by this macro.

The vector struct will always be `repr(C)`, and this cannot be changed.
The vector enum must specify a `repr` type via the macro, which should be an integer type.
Enum discriminants may be specified after the field names.

## Examples

The following code:
```rust
vector_type!{
  // The vector struct's name
  pub struct Languages;

  // Attributes are allowed, though many traits are already derived
  #[derive(Default)]
  // The vector enum's name, and its repr type
  pub enum Language as u8;

  // Each field, and its respective enum variant
  abstract {
    /// English
    en: #[default] En,
    /// Spanish
    es: Es,
    /// Portuguese
    pt: Pt,
    /// French
    fr: Fr,
    /// German
    de: De,
    /// Russian
    ru: Ru,
    /// Chinese
    zh: Zh
  }
}
```

Will expand to something (roughly) like this:
```rust
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Languages<T> {
  /// English
  en: T,
  /// Spanish
  es: T,
  /// Portuguese
  pt: T,
  /// French
  fr: T,
  /// German
  de: T,
  /// Russian
  ru: T,
  /// Chinese
  zh: T
}

#[derive(Default)]
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Language {
  #[default] En, Es, Pt, Fr, De, Ru, Zh
}

// (implementations omitted)
```

You can then use it like so:
```rust
let lang_string: Languages<&'static str> = Languages {
  en: "Enable",
  es: "Habilitar",
  pt: "Habilitar",
  fr: "Activer",
  de: "Aktivieren",
  ru: "Включить",
  zh: "启用"
};

assert_eq!(lang_string[Language::En], "Enable");
assert_eq!(lang_string[Language::Fr], "Activer");
```

## Implementation
The following items are implemented for the vector struct and the vector enum:
```rust
impl<T> $VectorStruct<T> {
  const fn splat(value: T) -> Self where T: Copy;
  const fn get(&self, variant: $VectorEnum) -> &T;
  const fn get_mut(&mut self, variant: $VectorEnum) -> &mut T;
  const fn set(&mut self, variant: $VectorEnum, value: T) -> T;
  const fn zip<U>(self, other: $VectorStruct<U>) -> $VectorStruct<(T, U)>;
  fn zip_with<U, V>(self, other: $VectorStruct<U>, mut f: impl FnMut(T, U) -> V) -> $VectorStruct<V>;
  fn map<U>(self, mut f: impl FnMut(T) -> U) -> $VectorStruct<U>;
  fn map_tagged<U>(self, mut f: impl FnMut($VectorEnum, T) -> U) -> $VectorStruct<U>;
  fn try_map_opt<U>(self, mut f: impl FnMut(T) -> Option<U>) -> Option<$VectorStruct<U>>;
  fn try_map_res<U, E>(self, mut f: impl FnMut(T) -> Result<U, E>) -> Result<$VectorStruct<U>, E>;
  fn from_fn(f: impl FnMut($VectorEnum) -> T) -> Self;
  fn try_from_fn_opt(f: impl FnMut($VectorEnum) -> Option<T>) -> Option<Self>;
  fn try_from_fn_res<E>(f: impl FnMut($VectorEnum) -> Result<T, E>) -> Result<Self, E>;
  fn convert<U>(self) -> $VectorStruct<U> where T: Into<U>;
  fn try_convert<U>(self) -> Result<$VectorStruct<U>, T::Error> where T: TryInto<U>;
  const fn from_array(array: [T; $VectorEnum::VARIANTS_COUNT]) -> Self;
  const fn from_array_ref(array: &[T; $VectorEnum::VARIANTS_COUNT]) -> &Self;
  const fn from_array_ref_mut(array: &mut [T; $VectorEnum::VARIANTS_COUNT]) -> &mut Self;
  const fn into_array(self) -> [T; $VectorEnum::VARIANTS_COUNT];
  const fn as_array_ref(&self) -> &[T; $VectorEnum::VARIANTS_COUNT];
  const fn as_array_ref_mut(&mut self) -> &mut [T; $VectorEnum::VARIANTS_COUNT];
  const fn as_slice(&self) -> &[T];
  const fn as_slice_mut(&mut self) -> &mut [T];
  const fn each_ref(&self) -> $VectorStruct<&T>;
  const fn each_mut(&mut self) -> $VectorStruct<&mut T>;
  const fn each_ref_array(&self) -> [&T; $VectorEnum::VARIANTS_COUNT];
  const fn each_mut_array(&mut self) -> [&mut T; $VectorEnum::VARIANTS_COUNT];
  fn test_all(&self, f: impl FnMut(&T) -> bool) -> bool;
  fn test_any(&self, f: impl FnMut(&T) -> bool) -> bool;
  fn iter(&self) -> impl Iterator<Item = &T>;
  fn iter_mut(&mut self) -> impl Iterator<Item = &mut T>;
  fn iter_tagged(&self) -> impl Iterator<Item = ($VectorEnum, &T)>;
  fn iter_tagged_mut(&mut self) -> impl Iterator<Item = ($VectorEnum, &mut T)>;
  fn into_iter_tagged(self) -> impl Iterator<Item = ($VectorEnum, T);
}

impl $VectorStruct<bool> {
  const ALL_TRUE: Self;
  const ALL_FALSE: Self;
  const fn just(which: $VectorEnum) -> Self;
  const fn is_all_true(self) -> bool;
  const fn is_all_false(self) -> bool;
  const fn is_any_true(self) -> bool;
  const fn is_any_false(self) -> bool;
  const fn bool_and(self, other: Self) -> Self;
  const fn bool_xor(self, other: Self) -> Self;
  const fn bool_or(self, other: Self) -> Self;
  const fn bool_not(self) -> Self;
}

impl<T> Debug for $VectorStruct<T> where T: Debug;
impl<T> Clone for $VectorStruct<T> where T: Clone;
impl<T> Copy for $VectorStruct<T> where T: Copy;
impl<T> PartialEq for $VectorStruct<T> where T: PartialEq;
impl<T> Eq for $VectorStruct<T> where T: Eq;
impl<T> Hash for $VectorStruct<T> where T: Hash;

impl<T> AsRef<[T; $VectorEnum::VARIANTS_COUNT]> for $VectorStruct<T>;
impl<T> AsRef<[T]> for $VectorStruct<T>;
impl<T> AsMut<[T; $VectorEnum::VARIANTS_COUNT]> for $VectorStruct<T>;
impl<T> AsMut<[T]> for $VectorStruct<T>;
impl<T> From<[T; $VectorEnum::VARIANTS_COUNT]> for $VectorStruct<T>;
impl<T> From<$VectorStruct<T>> for [T; $VectorEnum::VARIANTS_COUNT];
impl<T> Index<$VectorEnum> for $VectorStruct<T>;
impl<T> IndexMut<$VectorEnum> for $VectorStruct<T>;
impl<T> IntoIterator for $VectorStruct<T>;
impl<'a, T> IntoIterator for &'a $VectorStruct<T>;
impl<'a, T> IntoIterator for &'a mut $VectorStruct<T>;

impl<Lhs, Rhs> Add<$VectorStruct<Rhs>> for $VectorStruct<Lhs> where Lhs: Add<Rhs>;
impl<Lhs, Rhs> Sub<$VectorStruct<Rhs>> for $VectorStruct<Lhs> where Lhs: Add<Rhs>;
impl<Lhs, Rhs> Mul<$VectorStruct<Rhs>> for $VectorStruct<Lhs> where Lhs: Add<Rhs>;
impl<Lhs, Rhs> Div<$VectorStruct<Rhs>> for $VectorStruct<Lhs> where Lhs: Add<Rhs>;
impl<Lhs, Rhs> Rem<$VectorStruct<Rhs>> for $VectorStruct<Lhs> where Lhs: Add<Rhs>;
impl<Lhs, Rhs> AddAssign<$VectorStruct<Rhs>> for $VectorStruct<Lhs> where Lhs: AddAssign<Rhs>;
impl<Lhs, Rhs> SubAssign<$VectorStruct<Rhs>> for $VectorStruct<Lhs> where Lhs: AddAssign<Rhs>;
impl<Lhs, Rhs> MulAssign<$VectorStruct<Rhs>> for $VectorStruct<Lhs> where Lhs: AddAssign<Rhs>;
impl<Lhs, Rhs> DivAssign<$VectorStruct<Rhs>> for $VectorStruct<Lhs> where Lhs: AddAssign<Rhs>;
impl<Lhs, Rhs> RemAssign<$VectorStruct<Rhs>> for $VectorStruct<Lhs> where Lhs: AddAssign<Rhs>;
impl<T> Neg for $VectorStruct<T> where T: Neg;

impl<Lhs, Rhs> BitAnd<$VectorStruct<Rhs>> for $VectorStruct<Lhs> where Lhs: Add<Rhs>;
impl<Lhs, Rhs> BitXor<$VectorStruct<Rhs>> for $VectorStruct<Lhs> where Lhs: Add<Rhs>;
impl<Lhs, Rhs> BitOr<$VectorStruct<Rhs>> for $VectorStruct<Lhs> where Lhs: Add<Rhs>;
impl<Lhs, Rhs> BitAndAssign<$VectorStruct<Rhs>> for $VectorStruct<Lhs> where Lhs: AddAssign<Rhs>;
impl<Lhs, Rhs> BitXorAssign<$VectorStruct<Rhs>> for $VectorStruct<Lhs> where Lhs: AddAssign<Rhs>;
impl<Lhs, Rhs> BitOrAssign<$VectorStruct<Rhs>> for $VectorStruct<Lhs> where Lhs: AddAssign<Rhs>;
impl<T> Not for $VectorStruct<T> where T: Not;

impl<Lhs, Rhs> Shl<$VectorStruct<Rhs>> for $VectorStruct<Lhs> where Lhs: Add<Rhs>;
impl<Lhs, Rhs> Shr<$VectorStruct<Rhs>> for $VectorStruct<Lhs> where Lhs: Add<Rhs>;
impl<Lhs, Rhs> ShlAssign<$VectorStruct<Rhs>> for $VectorStruct<Lhs> where Lhs: AddAssign<Rhs>;
impl<Lhs, Rhs> ShrAssign<$VectorStruct<Rhs>> for $VectorStruct<Lhs> where Lhs: AddAssign<Rhs>;

impl $VectorEnum {
  const VARIANTS_COUNT: usize;
  const VARIANTS_ARRAY: [Self; $VectorEnum::VARIANTS_COUNT];
  const VARIANTS: $VectorStruct<Self>;
  const fn to_num(self) -> $repr_type;
}

impl Debug for $VectorEnum;
impl Clone for $VectorEnum;
impl Copy for $VectorEnum;
impl PartialEq for $VectorEnum;
impl Eq for $VectorEnum;
impl PartialOrd for $VectorEnum;
impl Ord for $VectorEnum;
impl Hash for $VectorEnum;
```

### Caveats

By default, vector structs aren't given an implementation for `PartialOrd` or `Ord`.
This is done for the same reasons described [here](https://github.com/bitshifter/glam-rs/issues/138).
If you need `PartialOrd` or `Ord` for your vector struct, you can easily add a derive for them.

As a consequence, vector structs also do not implement `Borrow` or `BorrowMut`,
as they require `Eq`, `Ord` and `Hash` to be equivalent over the borrowed and owned types.
If you need `Borrow` or `BorrowMut` for your vector struct, you can implement them manually.
