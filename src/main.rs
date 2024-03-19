struct Student {
    _name: String,
    _age: u8,
    _sex: char,
}


fn main() {
    let _s_1 = Student {
        _name: String::from("Tom"),
        _age: 20,
        _sex: 'M',
    };
    
    let _s_2 = Student {
        _name: String::from("Jerry"),
        _age: 18,
        _sex: 'F',
    };
}
