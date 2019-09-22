use std::convert::{TryFrom, TryInto};

#[derive(Clone, Debug)]
struct User {
    name: String,
    phones: Vec<Phone>,
}

#[derive(Clone, Debug)]
struct Phone {
    os: String,
    model: String,
}

#[derive(Clone, Debug)]
struct PhoneTable {
    user: String,
    os: String,
    model: String,
}

impl TryFrom<User> for Vec<PhoneTable> {
    type Error = &'static str;

    fn try_from(v: User) -> Result<Self, Self::Error> {
        let name = v.name.clone();
        Ok(v.clone()
            .phones
            .into_iter()
            .map(|p| PhoneTable {
                user: name.clone(),
                os: p.os,
                model: p.model,
            })
            .collect())
    }
}

type Hoge = Vec<PhoneTable>;

#[test]
fn test_convert_vector() {
    let my_phone_1 = Phone {
        os: "ios".to_string(),
        model: "iPhoneSE".to_string(),
    };

    let my_phone_2 = Phone {
        os: "android".to_string(),
        model: "Pixel3".to_string(),
    };

    let user = User {
        name: "Taro".to_string(),
        phones: vec![my_phone_1, my_phone_2],
    };

    // let phone_table: Result<Vec<PhoneTable>, _> = user.try_into();

    let phone_table = Hoge::try_from(user);
    println!("{:#?}", phone_table);
    // Output
    // Ok(
    //     [
    //         PhoneTable {
    //             user: "Taro",
    //             os: "ios",
    //             model: "iPhoneSE",
    //         },
    //         PhoneTable {
    //             user: "Taro",
    //             os: "android",
    //             model: "Pixel3",
    //         },
    //     ],
    // )
}
