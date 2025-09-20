use projup::data::{get_words, to_case, Cases};

#[test]
fn get_words_test()
{
    let source = "Hello **- 123world_beansAREcOOL555";
    let words = get_words(source);
    let expect = vec!["Hello", "123", "world", "beans", "ARE", "cOOL", "555"];
    assert_eq!(words, expect);
}
#[test]
fn camel_test()
{
    let source = vec!["Hello", "123", "world", "beans", "ARE", "cOOL", "555"];
    let str = to_case(source, Cases::Camel);
    assert_eq!(str, "hello123WorldBeansAreCool555");
}
#[test]
fn pascal_test()
{
    let source = vec!["Hello", "123", "world", "beans", "ARE", "cOOL", "555"];
    let str = to_case(source, Cases::Pascal);
    assert_eq!(str, "Hello123WorldBeansAreCool555");
}
#[test]
fn snake_test()
{
    let source = vec!["Hello", "123", "world", "beans", "ARE", "cOOL", "555"];
    let str = to_case(source, Cases::Snake);
    assert_eq!(str, "hello_123_world_beans_are_cool_555");
}
#[test]
fn leading_number()
{
    let source = vec!["1", "Hello", "123", "world", "beans", "ARE", "cOOL", "555"];
    let str = to_case(source, Cases::Snake);
    assert_eq!(str, "_1_hello_123_world_beans_are_cool_555");
}
#[test]
fn camel_snake_test()
{
    let source = vec!["Hello", "123", "world", "beans", "ARE", "cOOL", "555"];
    let str = to_case(source, Cases::CamelSnake);
    assert_eq!(str, "hello_123_World_Beans_Are_Cool_555");
}
#[test]
fn sentence_test()
{
    let source = vec!["Hello", "123", "world", "beans", "ARE", "cOOL", "555"];
    let str = to_case(source, Cases::Sentence);
    assert_eq!(str, "Hello 123 world beans are cool 555");
}