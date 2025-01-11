use service_builder::builder;

#[test]
fn test_getter() {
    #[builder]
    struct GetterTest {
        #[builder(getter)]
        field1: String,
        field2: i32,
    }

    let test = GetterTest::builder()
        .field1("test".to_string())
        .field2(42)
        .build()
        .unwrap();

    assert_eq!(test.get_field1(), &"test".to_string());
    // Verify field2 doesn't have a getter
    // This should fail to compile if uncommented:
    // test.get_field2();
}

#[test]
fn test_setter() {
    #[builder]
    struct SetterTest {
        #[builder(setter)]
        field1: String,
        field2: i32,
    }

    let mut test = SetterTest::builder()
        .field1("test".to_string())
        .field2(42)
        .build()
        .unwrap();

    test.set_field1("new value".to_string());
    assert_eq!(test.field1, "new value".to_string());
    // Verify field2 doesn't have a setter
    // This should fail to compile if uncommented:
    // test.set_field2(100);
}

#[test]
fn test_getter_and_setter() {
    #[builder]
    struct GetterSetterTest {
        #[builder(getter, setter)]
        field1: String,
        #[builder(getter, setter)]
        field2: i32,
    }

    let mut test = GetterSetterTest::builder()
        .field1("test".to_string())
        .field2(42)
        .build()
        .unwrap();

    // Test initial values via getters
    assert_eq!(test.get_field1(), &"test".to_string());
    assert_eq!(test.get_field2(), &42);

    // Test setters
    test.set_field1("new value".to_string());
    test.set_field2(100);

    // Verify new values via getters
    assert_eq!(test.get_field1(), &"new value".to_string());
    assert_eq!(test.get_field2(), &100);
}

#[test]
fn test_multiple_fields() {
    #[builder]
    struct MultiFieldTest {
        #[builder(getter)]
        string_field: String,
        #[builder(setter)]
        int_field: i32,
        #[builder(getter, setter)]
        bool_field: bool,
        plain_field: f64,
    }

    let mut test = MultiFieldTest::builder()
        .string_field("test".to_string())
        .int_field(42)
        .bool_field(true)
        .plain_field(3.14)
        .build()
        .unwrap();

    // Test getters
    assert_eq!(test.get_string_field(), &"test".to_string());
    assert_eq!(test.get_bool_field(), &true);

    // Test setters
    test.set_int_field(100);
    test.set_bool_field(false);

    // Verify changes
    assert_eq!(test.int_field, 100);
    assert_eq!(test.get_bool_field(), &false);

    // These should fail to compile if uncommented:
    // test.set_string_field("fail".to_string());
    // test.get_int_field();
    // test.get_plain_field();
    // test.set_plain_field(2.718);
}