/// Multiplies two integers
#[must_use]
pub const fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

#[cfg(test)]
mod test {
    use super::*;
    use demonstrate::demonstrate;
    use pretty_assertions::{assert_eq as pretty_assert_eq, assert_ne as pretty_assert_ne};

    #[cfg(target_arch = "wasm32")]
    use wasm_bindgen_test::wasm_bindgen_test;

    demonstrate! {
        describe "module" {
            use super::*;

            before {
                let four = 4;
            }

            #[should_panic]
            it "can fail" {
                pretty_assert_ne!(multiply(2, 2), four);
            }

            test "is returnable" -> Result<(), &'static str> {
                if multiply(2, 2) == four {
                    Ok(())
                } else {
                    Err("It isn't 4! :o")
                }
            }

            // Native async tests
            #[cfg(not(target_arch = "wasm32"))]
            context "native async" {
                before {
                    let is_4_task = smol::spawn(async {
                        multiply(2, 2)
                    });
                }

                async it "awaits" {
                    pretty_assert_eq!(four, is_4_task.await);
                }
            }

            // WASM async tests (wasm-bindgen-test)
            #[cfg(target_arch = "wasm32")]
            context "wasm async" {
                before {
                    let future_value = async { multiply(2, 2) };
                }

                #[wasm_bindgen_test]
                async it "awaits in wasm" {
                    pretty_assert_eq!(four, future_value.await);
                }
            }
        }
    }
}
