/* 
구현 내용
 -  프로그램 실행시 터미널에 실행 가능 메뉴를 보여주고, 해당 번호 입력시 관련 작업 실행
 -  Data 추가, 삭제, 검색, 전체 내용 출력, 
 -  파일에서 해당 data 가져오기 및 현재 작업 data 파일에 저장
 -  'q' 를 입력시 프로그램 종료

구현실패 및 추후 구현필요 사항 
 -  주소록 vector 의 전체 내용을 clear 하는 기능을 구현하고자 하였으나, 
    std::vec 에 있는 clear 기능을 현재 vector 에 trait 및 generic 의 기능을 적용하는것에 실패함.
 -  파일에 현재 작업한 내용을 저장할 때, 단순히 작업내용을 append 하는 형식으로만 저장
    append 와 overwrite 을 구분하여 적용하는 것이 필요.
 -  한글로 파일 저장 및 가져오기 문제를 해결하지 못해서 영어로만 내용을 작성함. 
*/

#![allow(non_snake_case)]
#![allow(unused)]

use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::BufRead;
use std::io::{self, Write, Read};
use std::str;


/* ----- 키보드를 이용한 string 입력 받기------ */
pub fn keyboardInput() -> String
{
    let mut buffer = String::new();
    let stdin = io::stdin().read_line(&mut buffer).expect("typing error");
    buffer[..buffer.len()-2].to_string()
}

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
    // 1. 키보드 입력을 통한 이름 추가
    pub fn addPersonalInfo(&mut self)
    {
        println!("Please input a name and an Address.");
        println!("Name    : ");
        let inputName = keyboardInput();
        println!("Address : ");
        let inputAddress = keyboardInput();

        self.groupDatas.push(PersonalInfo::newPerson(inputName, inputAddress));
    }

    // 2. (동명이인이 없다고 가정) 이름을 검색하여 주소 찾기
    pub fn searchingPersonalInfo(&self)

    {
        println!("Please input a name to search.");
        println!("Name    : ");
        let inputName = keyboardInput();

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

    // 3. 현재 목록에 등록된 모든 이름 출력
    pub fn showGroupInfo(&self)
    {
        for data in self.groupDatas.iter()
        {
            data.showPersonalInfo();
        }
    }

    // 4. (동명이인이 없다고 가정) 이름으로 검색하여 삭제하기 
    pub fn removePersonalInfo(&mut self)
    {
        println!("Please input a name to delete.");
        println!("Name    : ");
        let inputName = keyboardInput();

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

    // 5. 저장된 txt file 을 읽어서 해당 data 를 연락처 vector 값으로 가져오기
    pub fn loadGroupInfoFromFile(&mut self, fileName:&str)
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
            self.groupDatas.push(PersonalInfo::newPerson(name, address));

            contents = contents[cuttingIdx2+1..].to_string();
        }
    }

    // 6. file 에 연락처 추가 저장하기 
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
        println!("The file is updated!");
    }
}

/* ----- MAIN ----- */
fn main() {
    let mut group1 = GroupInfo{groupDatas: Vec::new()};
    let fileName  = "addressBook.txt";

    loop
    {
        println!("-----------------------------------------------------------------");
        println!("Please enter the number of the job you want.");
        println!("1. Add a new personal infomation.");
        println!("2. Search one memorized personal information.");
        println!("3. Check the all memorized personal information.");
        println!("4. Remove one memorized personal information.");
        println!("5. Load the address record from a file.");
        println!("6. Save the added address record to a file.");
        println!("Please enter 'q' if you want to quit this program.");
        println!("-----------------------------------------------------------------");

        let mut orderNumber = String::new();
        let stdin = io::stdin().read_line(&mut orderNumber).expect("typing error");
        if orderNumber.len() != 3
        {
            println!("Please enter one character among the vaild keys (1 - 6, q).");
        }
        else 
        {
            orderNumber = orderNumber[..1].to_string();
            
            if orderNumber == "1"  // input a new personal infomation. 
            {
                group1.addPersonalInfo();
            }
            else if orderNumber == "2" // Search one memorized personal information.
            {
                group1.searchingPersonalInfo();
            }
            else if orderNumber == "3" // Check the all memorized personal information.
            {
                group1.showGroupInfo();
            }
            else if orderNumber == "4" // Remove one memorized personal information.
            {
                group1.removePersonalInfo();
            }
            else if orderNumber == "5" // Load the address record from a file.
            {
                group1.loadGroupInfoFromFile(fileName);
            }
            else if orderNumber == "6" // Save the added address record to a file.
            {
                group1.addGroupInfoToFile(fileName);
            }
            else if orderNumber == "q" // quit this program.
            {
                println!("Bye!!"); 
                break;  
            }
            else 
            {
                println!("Please enter the valid key (1 - 6, q).");
            }
        }
    }
}
