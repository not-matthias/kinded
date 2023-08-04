#![allow(unused_imports)]
#![allow(dead_code)]

use kinded::Kinded;

#[derive(Kinded)]
enum Role {
    Guest,
    User(i32),
    #[allow(dead_code)]
    Admin {
        id: i32,
    },
}

mod base_enum {
    use super::*;

    mod fn_kind {
        use super::*;

        #[test]
        fn should_convert_unit_variant() {
            let guest = Role::Guest;
            assert_eq!(guest.kind(), RoleKind::Guest);
        }

        #[test]
        fn should_convert_unnamed_variant() {
            let user = Role::User(13);
            assert_eq!(user.kind(), RoleKind::User);
        }

        #[test]
        fn should_convert_named_variant() {
            let admin = Role::Admin { id: 404 };
            assert_eq!(admin.kind(), RoleKind::Admin);
        }
    }

    mod traits {
        use super::*;
        use kinded::Kinded;

        fn compute_kind<T: Kinded>(val: T) -> <T as Kinded>::Kind {
            val.kind()
        }

        #[test]
        fn should_implement_kinded() {
            let admin = Role::Admin { id: 32 };
            assert_eq!(compute_kind(admin), RoleKind::Admin);
        }
    }
}

mod kind_enum {
    mod traits {
        use super::super::{Role, RoleKind};

        #[test]
        fn should_implement_debug() {
            assert_eq!(format!("{:?}", RoleKind::Guest), "Guest")
        }

        #[test]
        fn should_implement_clone() {
            let _ = RoleKind::Admin.clone();
        }

        #[test]
        fn should_implement_copy() {
            fn receive_copy<T: Copy>() {}

            receive_copy::<RoleKind>();
        }

        #[test]
        fn should_implement_eq() {
            assert!(RoleKind::Guest.eq(&RoleKind::Guest));
            assert!(!RoleKind::Guest.eq(&RoleKind::User));
        }

        #[test]
        fn should_implement_from() {
            let user = Role::User(123);
            assert_eq!(RoleKind::from(user), RoleKind::User);
        }

        #[test]
        fn should_implement_from_ref() {
            let guest = Role::Guest;
            assert_eq!(RoleKind::from(&guest), RoleKind::Guest);
        }
    }
}

#[test]
fn should_allow_to_give_custom_name_kind_type() {
    #[derive(Kinded)]
    #[kinded(kind = SimpleDrink)]
    enum Drink {
        Tea(&'static str),
        Coffee(&'static str),
    }

    let green_tea = Drink::Tea("Green");
    assert_eq!(green_tea.kind(), SimpleDrink::Tea);
}

#[test]
fn should_allow_to_derive_custom_traits() {
    use std::collections::HashMap;

    #[derive(Kinded)]
    #[kinded(derive(Hash, Eq))]
    enum Drink {
        Tea(&'static str),
        Coffee(&'static str),
    }

    let mut drinks = HashMap::new();
    drinks.insert(DrinkKind::Tea, 5);
}

#[test]
fn should_work_with_generics() {
    use std::collections::HashMap;

    #[derive(Kinded)]
    enum Maybe<T> {
        Just(T),
        Nothing,
    }

    assert_eq!(Maybe::Just(13).kind(), MaybeKind::Just);
}

#[test]
fn should_work_with_lifetimes() {
    use std::collections::HashMap;

    #[derive(Kinded)]
    enum Identifier<'a, I> {
        Name(&'a str),
        Id(I),
    }

    let identifier: Identifier<i32> = Identifier::Name("Xen");
    assert_eq!(identifier.kind(), IdentifierKind::Name);
}