#![allow(non_snake_case)]
#![allow(unused)]

use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::{self, Write, Read};
use std::str;

/* ----- 저장될 Data 를 정의한 구조체 및 관련 methods  ------ */

struct PersonalInfo 
{
    name : String,  
    address : String, 
}

impl PersonalInfo
{
    //static method (해당 구조체 instance 생성자)
    pub fn newPerson(inputName: String, inputAddress : String) -> Self // method  return : Self struct 명(PersonalInfo))과 동일 
    {
        PersonalInfo{name:inputName, address:inputAddress}  // Self{} 로도 표현 가능
     }

    pub fn showPersonalInfo(&self)
    {
        println!("[NAME : {}  / ADDRESS : {}]", &self.name, &self.address);  
    }
}

/* ----- 연락처 Data 를 저장, 관리에 필요한 구조체 정의 및 methods ----- */
pub struct GroupInfo
{
    groupDatas : Vec<PersonalInfo>,
}

impl GroupInfo
{
    // 목록에 이름을 추가
    pub fn addPersonalInfo(&mut self, inputName:String, inputAddress:String)
    {
        self.groupDatas.push(PersonalInfo::newPerson(inputName, inputAddress));
    }

    // (동명이인이 없다고 가정) 이름으로 검색하여 삭제하기 
    pub fn removePersonalInfo(&mut self, inputName:String)
    {
        let mut searchingStatus = false;
        let mut cnt:usize = 0;
        for person in self.groupDatas.iter()
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
            self.groupDatas.remove(cnt); // vector index 값을 찾는 method 를 못찾아서 위 반복문을 통해 얻음.
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

        for person in self.groupDatas.iter()
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
    pub fn showGroupInfo(&self)
    {
        for data in self.groupDatas.iter()
        {
            data.showPersonalInfo();
        }
    }

    // 입력된 file 에 연락처 저장 ()
    pub fn addGroupInfoToFile(&self, fileName:&str)
    {
        let mut file = OpenOptions::new().append(true).open(fileName);
        let mut file = match file{
            Ok(File) => File,
            Err(error) => panic!("There was a problem opening the file: {:?}", error),
        };
        for data in self.groupDatas.iter()
        {
            let writingInfo = format!("{}\t{}\n", data.name, data.address);
            file.write_all(writingInfo.as_bytes()).expect("Writing error");
        }
        
    }

    // 저장된 txt file 을 읽어서 해당 data 를 연락처 vector 값으로 저장
    pub fn readGroupInfoFromFile(&mut self, fileName:&str)
    {
        let mut contents = fs::read_to_string(fileName);
        let mut contents = match contents{
            Ok(String) => String,
            Err(error) => panic!("There was a problem opening the file: {:?}", error),
        };
        while contents.chars().count() > 0
        {
            let cuttingIdx1 = contents.find('\t').unwrap();
            let cuttingIdx2 = contents.find('\n').unwrap();
            let name = contents[..cuttingIdx1].to_string();
            let address = contents[cuttingIdx1+1..cuttingIdx2].to_string();
            self.addPersonalInfo(name, address);

            contents = contents[cuttingIdx2+1..].to_string();
        }
        
    }
}
    // Vector 내 저장된 data 모두 삭제 -> std method 에 대한 trait 사용 공부 필요
    // trait Clear{}
    // pub fn clearGroupInfo(&self) {}

/* ----- MAIN ----- */
fn main() {
    /* ----- 연락처 그룹 vector 생성 및 data 입력 ----- */
    let mut group1 = GroupInfo{groupDatas: Vec::new()};

    group1.addPersonalInfo("HJ".to_string(), "USA".to_string());
    group1.addPersonalInfo("DH".to_string(), "UK".to_string());
    group1.addPersonalInfo("JM".to_string(), "France".to_string());
    group1.addPersonalInfo("YG".to_string(), "Korea".to_string());
   
    group1.showGroupInfo();
    group1.searchingPersonalInfo("YG".to_string());
    group1.searchingPersonalInfo("aa".to_string());
    group1.removePersonalInfo("YG".to_string());
    group1.showGroupInfo();
    group1.searchingPersonalInfo("YG".to_string());

    /* ----- txt file 에 해당 data 저장 ----- */
    let fileName  = "addressBook.txt";
    group1.addGroupInfoToFile(fileName);

    /* ----- txt file 에 해당 data 저장 ----- */
    println!("------- readGroup---------");
    let mut readGroup = GroupInfo{groupDatas:Vec::new()};
    readGroup.readGroupInfoFromFile(&fileName);
    readGroup.showGroupInfo();

}





/* -----  ----- */
/* -----  ----- */


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


