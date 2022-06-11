fn main() {
    impl<A: TraitB + TraitC, D: TraitE + TraitF> MyTrait<A, D> for YourType {}

    // Expressing bounds with a `where` clause
    impl<A, D> MyTrait<A, D> for YourType
    where
        A: TraitB + TraitC,
        D: TraitE + TraitF,
    {
    }
}
