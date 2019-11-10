This is one possible way to set different [serde](https://github.com/serde-rs/serde) configurations for the same structure. This went full non-idiomatic.

Motivation:

- Having different pre-existing APIs refering to the same structure.
- Related issue: https://github.com/serde-rs/serde/issues/1443

---

For [this demonstration](src/lib.rs), two different JSON APIs were used.

but first, assume this single underlying object that shall represent both APIs:

```rust
struct A {
    a: bool, // both APIs do rename this to "fallback_for_all"
    b: B, // one API flattens this, the other doesn't
    c: bool, // one API renames this, the other doesn't
}
struct B {
    ba: bool, // one API renames this, the other doesn't
}
```

Now the &str example for the APIs:

API 1:
```json
{
    "fallback_for_all": true,
    "serde1_ba": true,
    "c": true
}
```

API 2:
```json
{
    "fallback_for_all": true,
    "b": {
        "serde2_ba": true
    },
    "only_for_serde2": true
}
```

So the intention here is to have the same type of object (`A`) to be able to get constructed from the deserialization of both APIs. Since they are the same type of object, a single object definition is required. This means that maintanence is reduced, and multiple objects may be comparable/clonable to each other (assuming the traits were implemented), and so on..

But this isn't for free. When you skim at the [Cargo.toml](Cargo.toml) you will see it.  
For my particular use-case, which is having multiple APIs in multiple data formats for an enormous structure, it does pays off.

---

The final intention is to share this possibility and hopefully see better ways to do it 0/

---

The "design":

- Multiple serde were required because multiple `Deserialize` implementation were required for the same object. So yeah.. `serde` and `serde_derive` are built multiple times (but their dependencies are built only once).
    - The same version of `serde` can be used, even if they are at the same commit. They only need to diverge on the branch name.
- Multiple data formats were also required, because each needed to be tied to a particular `serde::Deserialize` trait implementation.
- Some changes to `serde` itself were made, namely:
    - [Adding](https://github.com/swfsql/serde/blob/c183625898a329b5bfd2c3cb8a18f3010adb9aeb/serde_derive/src/lib.rs#L81) `serde1/2` attribute into the `proc_macro_derive`s gate. 
    - [Adding](https://github.com/swfsql/serde/blob/c183625898a329b5bfd2c3cb8a18f3010adb9aeb/serde_derive/src/internals/symbol.rs#L26) them also for when reading the attributes.
    - And some other minor changes (which only makes sense it this ad-hoc use-case, unfortunately).
