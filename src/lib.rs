#[macro_export]
macro_rules! genawaiter_iterator {
    (struct $iterator_struct_name:ident yields $item_type_path:path) => {
        struct $iterator_struct_name(
            std::pin::Pin<Box<dyn genawaiter::Generator<Yield = $item_type_path, Return = ()>>>,
        );

        impl Iterator for $iterator_struct_name {
            type Item = $item_type_path;

            fn next(&mut self) -> Option<Self::Item> {
                match self.0.as_mut().resume() {
                    genawaiter::GeneratorState::Yielded(value) => Some(value),
                    genawaiter::GeneratorState::Complete(()) => None,
                }
            }
        }

        impl<T: genawaiter::Generator<Yield = $item_type_path, Return = ()> + 'static> From<T>
            for $iterator_struct_name
        {
            fn from(generator: T) -> Self {
                Self(Box::pin(generator))
            }
        }
    };
}

#[macro_export]
macro_rules! gen {
    ($content:block) => {
        genawaiter::sync::gen!($content).into()
    };
}

#[cfg(test)]
mod tests {
    use genawaiter::yield_;

    #[test]
    fn it_works() {
        genawaiter_iterator!(struct Iter yields usize);

        fn iter() -> Iter {
            gen!({
                yield_!(1);
                yield_!(2);
                yield_!(3);
            })
        }

        let mut iterator_instance = iter();
        assert_eq!(iterator_instance.next(), Some(1));
        assert_eq!(iterator_instance.next(), Some(2));
        assert_eq!(iterator_instance.next(), Some(3));
        assert_eq!(iterator_instance.next(), None);
    }
}
