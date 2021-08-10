// vect<T>
pub mod vectors_module {
    pub fn vectors_func(){
        // creation:
        let mut vector1: Vec<i32> = Vec::new();
        // macro vect:
        let mut vector2 = vec![1, 2, 3];

        // add:
        vector1.push(1);
        vector2.push(3);
        
        // read:

        let mut v = vec![1, 2, 3, 4, 5];
        let third = {
            &v[2];
        };

        v.push(5);
        match v.get(2){
            Some(third) => println!("The value is {}", third),
            None => (),
        }

    }

    pub fn iter_vectors(){
        let v: Vec<i32> = vec![0, 2, 4, 6, 8, 10];
        for index in &v {
            println!("{}", index);
        }
        // iter over reference
        let mut vector: Vec<i32> = vec![1, 2, 3];
        for i in &mut vector{
            *i += 50
        }
    }
    pub fn differente_enum_types(){
        #[derive(Debug)]
        enum SpreadsheetCell {
            int(i8),
            float(f64),
            text(String),
        }
        let row = vec![
            SpreadsheetCell::int(3),
            SpreadsheetCell::float(5.4),
            SpreadsheetCell::text(String::from("Blue"))
        ];
        println!("{:?}", row[2]);
    }
}