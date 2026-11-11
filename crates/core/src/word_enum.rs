//! The lists of words of the language (keywords, built-in functions, colors…): each is an
//! enum that knows how its values are written.

/// Defines an enum of words, each variant with the word it is written as, and gives it
/// `ALL`, `from_word` and `as_str`.
///
/// One place lists the variants and their words, so that a new word can't be forgotten in
/// `ALL` or in one of the two conversions. Two variants written as the same word don't
/// compile: the second is a pattern that is never reached.
macro_rules! word_enum {
    (
        $(#[$meta:meta])*
        $visibility:vis enum $name:ident {
            $(
                $(#[$variant_meta:meta])*
                $variant:ident => $word:literal,
            )+
        }
    ) => {
        $(#[$meta])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        $visibility enum $name {
            $(
                $(#[$variant_meta])*
                $variant,
            )+
        }

        impl $name {
            /// Every value, in the order of the specifications.
            pub const ALL: [Self; [$(Self::$variant),+].len()] = [$(Self::$variant),+];

            /// The value written `word`, if it is one of them.
            #[must_use]
            pub fn from_word(word: &str) -> Option<Self> {
                #[deny(unreachable_patterns)]
                match word {
                    $($word => Some(Self::$variant),)+
                    _ => None,
                }
            }

            /// The value as it is written.
            #[must_use]
            pub const fn as_str(self) -> &'static str {
                match self {
                    $(Self::$variant => $word,)+
                }
            }
        }
    };
}

pub(crate) use word_enum;

#[cfg(test)]
mod tests {
    word_enum! {
        /// Two words, for the tests.
        enum Side {
            /// `left`
            Left => "left",
            /// `right`
            Right => "right",
        }
    }

    #[test]
    fn all_lists_every_value_in_order() {
        assert_eq!(Side::ALL, [Side::Left, Side::Right]);
    }

    #[test]
    fn every_value_reads_back_from_its_word() {
        for side in Side::ALL {
            assert_eq!(Side::from_word(side.as_str()), Some(side));
        }
    }

    #[test]
    fn another_word_is_none_of_them() {
        assert_eq!(Side::from_word("up"), None);
        assert_eq!(Side::from_word("Left"), None);
        assert_eq!(Side::from_word(""), None);
    }
}
