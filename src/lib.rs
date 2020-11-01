use ureq::{self};

const BASEURL: &str = "https://nekobot.xyz/api/";
const _DOCS: &str = "https://docs.nekobot.xyz/";
/* 
    The type of image to get. 
    Current types: hass, hmidriff, pgif, 4k, hentai, holo, hneko, neko, hkitsune, 
    kemonomimi, anal, hanal, gonewild, kanna, ass, pussy, thigh, 
    hthigh, gah, coffee, food, paizuri, tentacle, boobs, hboobs, 
    donator types: cosplay, swimsuit, pantsu 
*/
pub fn image(image_type: &str, auth: &str) -> ureq::SerdeValue {
    if auth == "" {
       let resp = ureq::get(&(BASEURL.to_owned() + "/image?type=" + image_type))
        .set("Content-Type", "application/json")
        .set("Authorization", auth).call().into_json().unwrap();
        return resp;
    } else {
        let resp =  ureq::get(&(BASEURL.to_owned() + "/image?type=" + image_type))
            .set("Content-Type", "application/json").call().into_json().unwrap();
        return resp["message"].to_owned();
    }
}

pub fn generate(generate: &str, text: &str, raw: &str) -> ureq::SerdeValue {
    if generate == "threats" {
        let resp =  ureq::get(&(BASEURL.to_owned()  + "imagegen?type=" + generate + "&url=" + text + "&raw=" + raw))
        .set("Content-Type", "application/json").call().into_json().unwrap();
        return resp["message"].to_owned();
    } else if generate == "baguette" {
        let resp =  ureq::get(&(BASEURL.to_owned() + "imagegen?type=" + generate + "&url=" + text + "&raw=" + raw))
        .set("Content-Type", "application/json").call().into_json().unwrap();
        return resp["message"].to_owned();
    } else if generate == "clyde" {
        let resp =  ureq::get(&(BASEURL.to_owned() + "imagegen?type=" + generate + "&text=" + text + "&raw=" + raw))
        .set("Content-Type", "application/json").call().into_json().unwrap();
        return resp["message"].to_owned();
    } else if generate == "ship" {
//1 Finish
    } else if generate == "captcha" {
// Finish
    } else if generate == "whowouldwin" {
//1 Finish
    } else if generate == "changemymind" {
        let resp =  ureq::get(&(BASEURL.to_owned() + "imagegen?type=" + generate + "&text=" + text + "&raw=" + raw))
        .set("Content-Type", "application/json").call().into_json().unwrap();
        return resp["message"].to_owned();
    } else if generate == "ddlc" {
// Finsih
    } else if generate == "jpeg" {
        let resp =  ureq::get(&(BASEURL.to_owned() + "imagegen?type=" + generate + "&url=" + text + "&raw=" + raw))
        .set("Content-Type", "application/json").call().into_json().unwrap();
        return resp["message"].to_owned();
    } else if generate == "lolice" {
        let resp =  ureq::get(&(BASEURL.to_owned() + "imagegen?type=" + generate + "&url=" + text + "&raw=" + raw))
        .set("Content-Type", "application/json").call().into_json().unwrap();
        return resp["message"].to_owned();
    } else if generate == "kannagen" {
        let resp =  ureq::get(&(BASEURL.to_owned() + "imagegen?type=" + generate + "&text=" + text + "&raw=" + raw))
        .set("Content-Type", "application/json").call().into_json().unwrap();
        return resp["message"].to_owned();
    } else if generate == "iphonex" {
        let resp =  ureq::get(&(BASEURL.to_owned() + "imagegen?type=" + generate + "&url=" + text + "&raw=" + raw))
        .set("Content-Type", "application/json").call().into_json().unwrap();
        return resp["message"].to_owned();
    } else if generate == "kms" {
        let resp =  ureq::get(&(BASEURL.to_owned() + "imagegen?type=" + generate + "&url=" + text))
        .set("Content-Type", "application/json").call().into_json().unwrap();
        return resp["message"].to_owned();
    } else if generate == "animeface" {
        let resp =  ureq::get(&(BASEURL.to_owned() + "imagegen?type=" + generate + "&image=" + text))
        .set("Content-Type", "application/json").call().into_json().unwrap();
        return resp["message"].to_owned();
    } else if generate == "trap" {
// Finish
    } else if generate == "nichijou" {
        let resp =  ureq::get(&(BASEURL.to_owned() + "imagegen?type=" + generate + "&text=" + text))
        .set("Content-Type", "application/json").call().into_json().unwrap();
        return resp["message"].to_owned();
    } else if generate == "awooify" {
        let resp =  ureq::get(&(BASEURL.to_owned() + "imagegen?type=" + generate + "&url=" + text + "&raw=" + raw))
        .set("Content-Type", "application/json").call().into_json().unwrap();
        return resp["message"].to_owned();
    } else if generate == "trumptweet" {
        let resp =  ureq::get(&(BASEURL.to_owned() + "imagegen?type=" + generate + "&text=" + text + "&raw=" + raw))
        .set("Content-Type", "application/json").call().into_json().unwrap();
        return resp["message"].to_owned();
    } else if generate == "tweet" {
// Finish
    } else if generate == "kidnap" {
        let resp =  ureq::get(&(BASEURL.to_owned() + "imagegen?type=" + generate + "&image=" + text))
        .set("Content-Type", "application/json").call().into_json().unwrap();
        return resp["message"].to_owned();
    } else if generate == "deepfry" {
        let resp =  ureq::get(&(BASEURL.to_owned() + "imagegen?type=" + generate + "&image=" + text + "&raw=" + raw))
        .set("Content-Type", "application/json").call().into_json().unwrap();
        return resp["message"].to_owned();
    } else if generate == "blurpify" {
        let resp =  ureq::get(&(BASEURL.to_owned() + "imagegen?type=" + generate + "&image=" + text + "&raw=" + raw))
        .set("Content-Type", "application/json").call().into_json().unwrap();
        return resp["message"].to_owned();
    } else if generate == "phcomment" {
// Finish
    } else if generate == "magik" {
// Finish
    } else if generate == "deepfry" {
        let resp =  ureq::get(&(BASEURL.to_owned() + "imagegen?type=" + generate + "&image=" + text + "&raw=" + raw))
        .set("Content-Type", "application/json").call().into_json().unwrap();
        return resp["message"].to_owned();
    } else if generate == "osu" {
// Finish
    } else if generate == "clickforhentai" { 
// Finish
    } else if generate == "fact" {
        let resp =  ureq::get(&(BASEURL.to_owned() + "imagegen?type=" + generate + "&text=" + text + "&raw=" + raw))
        .set("Content-Type", "application/json").call().into_json().unwrap();
        return resp["message"].to_owned();
    } else if generate == "trash" {
        let resp =  ureq::get(&(BASEURL.to_owned() + "imagegen?type=" + generate + "&url=" + text))
        .set("Content-Type", "application/json").call().into_json().unwrap();
        return resp["message"].to_owned();
    } else if generate == "stickbug" {
        let resp =  ureq::get(&(BASEURL.to_owned() + "imagegen?type=" + generate + "&url=" + text))
        .set("Content-Type", "application/json").call().into_json().unwrap();
        return resp["message"].to_owned();
    }
    let resp =  ureq::get(&(BASEURL.to_owned() + "/api"))
    .set("Content-Type", "application/json").call().into_json().unwrap();
    return resp;
}
/*
    Deprecated
*/
pub fn osu(key: &str, username: &str) -> ureq::SerdeValue {
    let resp =  ureq::get(&(BASEURL.to_owned() + "imagegen?type=osu&url=" + key + "&username=" + username))
    .set("Content-Type", "application/json").call().into_json().unwrap();
    return resp["message"].to_owned();
}

pub fn clickforhentai(image: &str, fontsize: &str) -> ureq::SerdeValue {
    let resp =  ureq::get(&(BASEURL.to_owned() + "imagegen?type=clickforhentai&image=" + image + "&fontsize=" + fontsize))
    .set("Content-Type", "application/json").call().into_json().unwrap();
    return resp["message"].to_owned();
}

pub fn magik(image: &str, intensity: &str, raw: &str) -> ureq::SerdeValue {
    let resp =  ureq::get(&(BASEURL.to_owned() + "imagegen?type=magik&image=" + image + "&intensity=" + intensity + "&raw=" + raw ))
    .set("Content-Type", "application/json").call().into_json().unwrap();
    return resp["message"].to_owned();
}

pub fn trap(name: &str, author: &str, image: &str, raw: &str) -> ureq::SerdeValue {
    let resp =  ureq::get(&(BASEURL.to_owned() + "imagegen?type=trip&image=" + image + "&name=" + name + "&author" + author + "&raw=" + raw ))
    .set("Content-Type", "application/json").call().into_json().unwrap();
    return resp["message"].to_owned();
}

pub fn captcha(username: &str, image: &str, raw: &str) -> ureq::SerdeValue {
    let resp =  ureq::get(&(BASEURL.to_owned() + "imagegen?type=trip&captcha=" + image + "&username=" + username + "&raw=" + raw ))
    .set("Content-Type", "application/json").call().into_json().unwrap();
    return resp["message"].to_owned();
}

pub fn tweet(username: &str,  text: &str, raw: &str) -> ureq::SerdeValue {
    let resp =  ureq::get(&(BASEURL.to_owned() + "imagegen?type=tweet&username=" + username + "&text=" + text + "&raw=" + raw))
    .set("Content-Type", "application/json").call().into_json().unwrap();
    return resp["message"].to_owned();
}

pub fn phcomment(image: &str, username: &str,  text: &str, raw: &str) -> ureq::SerdeValue {
    let resp =  ureq::get(&(BASEURL.to_owned() + "imagegen?type=phcomment&image=" + image + "&text=" + text + "&username=" + username + "&raw=" + raw))
    .set("Content-Type", "application/json").call().into_json().unwrap();
    return resp["message"].to_owned();
}
/* 
aracter	string	
Can be either monika, yuri, natsuki, sayori or m, y, n, s

background	string	
Background of the image, types: bedroom, class, closet, club, corridor, house, kitchen, residential, sayori_bedroom

body	string	
Body of the character, there is only 1 or 2 for monika and 1, 1b, 2, 2b for the rest

face	string	
Face of the character to go with the body, is best to just see all the types at https://github.com/hibikidesu/NekoBot/blob/master/modules/fun.py#L14 (line14 to 34)

text	string	
Text for the character to say, max length of 140
*/
pub fn ddlc(character: &str, background: &str,  body: &str, face: &str, text: &str,  raw: &str) -> ureq::SerdeValue {
    let resp =  ureq::get(&(BASEURL.to_owned() + "imagegen?type=ddlc&character=" +  character + "&background=" + background + "&text=" + text + "&body=" + body + "&face=" + face + "&raw=" + raw))
    .set("Content-Type", "application/json").call().into_json().unwrap();
    return resp["message"].to_owned();
}

pub fn whowouldwin_or_ship(generate: &str, user1: &str,  user2: &str, raw: &str) -> ureq::SerdeValue {
    if generate == "whowouldwin" {
        let resp =  ureq::get(&(BASEURL.to_owned() + "imagegen?type=whowouldwin&user1=" + user1 + "&user2=" + user2 + "&raw=" + raw))
        .set("Content-Type", "application/json").call().into_json().unwrap();
        return resp["message"].to_owned();
    } else if generate == "ship" {
        let resp =  ureq::get(&(BASEURL.to_owned() + "imagegen?type=ship&user1=" + user1 + "&user2=" + user2 + "&raw=" + raw))
        .set("Content-Type", "application/json").call().into_json().unwrap();
        return resp["message"].to_owned();
    } else {
        let resp =  ureq::get(&(BASEURL.to_owned() + "/api"))
        .set("Content-Type", "application/json").call().into_json().unwrap();
        return resp;       
    }
}