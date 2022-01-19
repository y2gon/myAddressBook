#![allow(non_snake_case)]
#![allow(unused)]

use std::fs::File;
use std::fs::OpenOptions;
use std::io::{self, Write, Read};
use std::str;

/* 파일 입출력 */

/* 저장될 Data 를 정의한 구조체 및 관련 methods */

struct PersonalInfo 
{
    // id : u32,
    name : String,    // &str 로 선언할 경우 lifetime 문제 발생. &str 은 sliced array[u8] dlrh
    address : String, // String 은 Vec<u8> 인데... array 타입이 오히려 길이가 정해져 있으므로 stack 에 저장되어 lifetime 문제가 없어야 되는거 아닌지..
}

impl PersonalInfo
{
    pub fn showPersonalInfo(&self)
    {
        println!("[NAME : {}  / ADDRESS : {}]", &self.name, &self.address);  // {}, 와 {:?} 차이는??
        // -> The question mark operator (?) unwraps valid values or returns errornous values, propagating them to the calling function. 
        // -> {:?} 로 사용되면 해당 출력 형태가 아닌 데이터 형태 그대로 출력 가능하게 해줌??? 
    }
}

/* 연락처 Data 를 저장, 관리에 필요한 구조체 정의 및 methods */
pub struct AllPersonalInfo
{
    allDatas : Vec<PersonalInfo>,
}

impl AllPersonalInfo
{
    // 목록에 이름을 추가
    pub fn addPersonalInfo(&mut self, inputName:String, inputAddress:String)
    {
        self.allDatas.push(PersonalInfo{name:inputName, address:inputAddress});
    }

    // (동명이인이 없다고 가정) 이름으로 검색하여 삭제하기 
    pub fn removePersonalInfo(&mut self, inputName:String)
    {
        let mut searchingStatus = false;
        let mut cnt:usize = 0;
        for person in self.allDatas.iter()
        {
            if person.name == inputName
            {
                searchingStatus = true;
                break
            }
            cnt += 1;
        }

        if searchingStatus
        {
            self.allDatas.remove(cnt); // vector index 값을 찾는 method 를 못찾아서 위 반복문을 통해 얻음.
            println!("[{}] 's infomation is removed in this group.", inputName);
        }
        else // 삭제 하고자 하는 이름이 vector 내에 없을 경우
        {
            println!("[{}] is not exist in this group.", inputName);
        } 
    }

    // (동명이인이 없다고 가정) 이름을 검색하여 주소 찾기
    pub fn searchingPersonalInfo(&self, inputName:String)
    {
        let mut searchingStatus = false;

        for person in self.allDatas.iter()
        {
            if person.name == inputName
            {
                person.showPersonalInfo();
                searchingStatus = true;
                break;
            }
        }
        // 찾고자 하는 이름이 해당 그룹에 없을 경우
        if !searchingStatus
        {
            println!("[{}] is not registered in this group.", inputName);
        }
    }

    // 현재 목록에 등록된 모든 이름 출력
    pub fn showAllPersonalInfo(&self)
    {
        for data in self.allDatas.iter()
        {
            data.showPersonalInfo();
        }
    }
}

fn main() {

    // PersonalInfo 구조체 연습
    let person1 = PersonalInfo{name:"QQ".to_string(), address:"Jeju".to_string()};
    let person2 = PersonalInfo{name:"WW".to_string(), address:"Dokdo".to_string()};

    person1.showPersonalInfo();
    person2.showPersonalInfo();
    // --------------------------------------------------------------------
    let mut group1 = AllPersonalInfo{allDatas: Vec::new()};

    group1.addPersonalInfo("HJ".to_string(), "USA".to_string());
    group1.addPersonalInfo("DH".to_string(), "UK".to_string());
    group1.addPersonalInfo("JM".to_string(), "France".to_string());
    group1.addPersonalInfo("YG".to_string(), "Korea".to_string());
   
    group1.showAllPersonalInfo();
    group1.searchingPersonalInfo("YG".to_string());
    group1.searchingPersonalInfo("aa".to_string());
    group1.removePersonalInfo("YG".to_string());
    group1.showAllPersonalInfo();
    group1.searchingPersonalInfo("YG".to_string());

}




/*  아래 코드는 계속적으로 정리 필요 (파일 입출력 관련)

fn main() {
    let mut status = true;

    let fileName  = "addressBook.txt";
    
    let mut file =  OpenOptions::new().append(true).open(fileName).expect("The file creating error");

    while status
    {
        
        println!("1. Input a name and an address");
        println!("2. Searching the address"); 

        let mut numInput = String::new();
        let mut stdinNum = io::stdin();
        stdinNum.read_line(&mut numInput).expect("Number input error");

        if &numInput == "1\r\n"
        {
            println!("Please input a name");
            let mut nameBuffer = String::new();
            let mut nameStdin = io::stdin();
            nameStdin.read_line(&mut nameBuffer).expect("Name input error");
            let name = &nameBuffer[..nameBuffer.len()-2];

            println!("please input an address");
            let mut addressBuffer = String::new();
            let mut addressStdin = io::stdin();
            addressStdin.read_line(&mut addressBuffer).expect("Address input error");
            let address = &addressBuffer[..&addressBuffer.len()-2];

        
           file.write_all(name.as_bytes()).expect("Writing error");
           file.write_all(b"\t").expect("Writing error");
           file.write_all(address.as_bytes()).expect("Writing error");
           file.write_all(b"\n").expect("Writing error");
            
        }
        else if &numInput == "2\r\n"
        {
            println!("Searching an address. Pleaes input searching name");

            let mut searchingBuffer = String::new();
            let mut searchingStdin = io::stdin();
            searchingStdin.read_line(&mut searchingBuffer).expect("Searching address error");
            let searchingName = &searchingBuffer[..&searchingBuffer.len()-2];

            let mut file = File::open("addressBook.txt").unwrap();
            let mut readingContents = String::new();
            file.read_to_string(&mut readingContents).expect("file reading error");
            let nameLocation = &readingContents.find(searchingName).expect("The name is not exist.");
            let slicingContents = &readingContents[nameLocation+searchingName.len() + 1..];

            let slicingEnd = slicingContents.find('\n').unwrap();
            let searchedAddress = &slicingContents[..slicingEnd-1];
            println!("{:?}", searchedAddress);

        }
        else if &numInput == "\r\n"
        {
            println!("========= END ========");
            status = false;
        } 
        else
        {
            println!("Invalid Input");   
        }
    }

}


*/


