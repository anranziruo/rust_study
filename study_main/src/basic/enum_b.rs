enum FruitColor {
    Red,
    Yellow,
    Pink,
}

enum FruitObj <'data>{
    Apple(AppleObj<'data>),
    Banana(BananaObj<'data>),
    Peach(PeachObj<'data>),
}

impl <'data> FruitObj <'data> {
    
    pub fn get_color(data: &'data [u8]) ->FruitColor{
        return FruitColor::Red;
    }

    pub fn parse_color(data: &'data [u8]){
        let object = match Self::get_color(data){
            FruitColor::Red =>{
                println!("red")
            },
            FruitColor::Yellow =>{
                println!("yellow")
            },
            FruitColor::Pink =>{
                println!("pink")
            },
        };
    }
}

struct AppleObj <'data>{
    data: &'data [u8],
}

impl <'data> AppleObj <'data> {
    pub fn f_color(data: &[u8]) -> FruitColor{
        return FruitColor::Red
    }
}



struct BananaObj<'data>{
    data: &'data [u8],
}

impl <'data> BananaObj <'data> {
    pub fn f_color(data: &[u8]) -> FruitColor{
        return FruitColor::Yellow
    }
}

struct PeachObj<'data>{
    data: &'data [u8],
}

impl <'data> PeachObj <'data> {
    pub fn f_color(data: &[u8]) -> FruitColor{
        return FruitColor::Pink
    }
}


#[test]
fn test_enum_obj(){
   let data: &[u8] = &[0u8];
   let apple_obj = FruitObj::parse_color(&data);
}
