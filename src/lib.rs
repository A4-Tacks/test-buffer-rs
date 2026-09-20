#![doc = include_str!("../README.md")]

/// Buffer used for testing, output buffer name and content during panic
#[derive(Clone, Default, Debug)]
pub struct TestBuffer {
    name: Option<String>,
    buffer: String,
}

impl Drop for TestBuffer {
    fn drop(&mut self) {
        if std::thread::panicking() {
            println!("{}", self.desc())
        }
    }
}

impl std::fmt::Display for TestBuffer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.buffer.fmt(f)
    }
}

macro_rules! impl_eqs {
    ($($ty:ty),*) => {
        $(
            impl PartialEq<$ty> for TestBuffer {
                fn eq(&self, other: &$ty) -> bool {
                    self.buffer == *other
                }
            }
            impl PartialEq<TestBuffer> for $ty {
                fn eq(&self, other: &TestBuffer) -> bool {
                    other.buffer == *self
                }
            }
        )*
    };
}
impl_eqs!(str, &str, String);

impl TestBuffer {
    /// Creates a unnamed new [`TestBuffer`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a named new [`TestBuffer`].
    pub fn with_name(name: impl ToString) -> Self {
        TestBuffer { name: Some(name.to_string()), buffer: String::new() }
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn as_str(&self) -> &str {
        &self.buffer
    }

    /// Returns the description of this [`TestBuffer`].
    ///
    /// When panic, this struct output description in drop
    pub fn desc(&self) -> String {
        let name = self.name().unwrap_or("<unnamed>");
        let kind = if std::thread::panicking() { " in panicking" } else { "" };
        format!("===> TestBuffer {name} content preview{kind} <===\n{}", self.buffer)
    }

    #[cfg(feature = "expect-test")]
    /// Quick method, uses for [`expect_test`]
    pub fn expect(&self, expect: expect_test::Expect) {
        expect.assert_eq(self.as_str());
    }

    #[cfg(feature = "expect-test")]
    /// Quick method, uses for [`expect_test`]
    pub fn expect_file(&self, expect: expect_test::ExpectFile) {
        expect.assert_eq(self.as_str());
    }

    /// Use `write!` call this method.
    ///
    /// # Panics
    ///
    /// Panics if `args.fmt()` panic.
    ///
    /// # Examples
    ///
    /// ```
    /// use test_buffer::TestBuffer;
    ///
    /// let mut test_buffer = TestBuffer::new();
    /// write!(test_buffer, "foo");
    /// write!(test_buffer, "bar");
    /// assert_eq!(test_buffer, "foobar");
    /// ```
    pub fn write_fmt(&mut self, args: std::fmt::Arguments<'_>) {
        std::fmt::Write::write_fmt(&mut self.buffer, args).unwrap()
    }
}

impl std::ops::DerefMut for TestBuffer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.buffer
    }
}

impl std::ops::Deref for TestBuffer {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.buffer
    }
}
