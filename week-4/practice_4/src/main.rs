fn main() {
    
    let fullname = "Chibudum John Umeh" ; 
    let Department = "Computer Science " ;
    let uni = "Pan Athlantic University" ;


    let mut school = "School of Science".to_string() ; 
    school.push_str("and Technology"); 


    println!("my name is:{:?}",fullname );

    println!(" the length of my fullname is {:?}",fullname.len());
    println!("i am a student of {:?} department ",Department );
    println!("{:?}",school );
    println!("{:?}",uni  );

}
