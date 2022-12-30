#[derive(Clone)]
enum Address{
    Address(Box<my_list>),
    Nil
}
#[derive(Clone)]
struct my_list{
    value:i32,
    next:Address
}

impl my_list{
    fn append(&mut self, data:i32){
        match self.next {
            //ref is used to get a copy of the value rather than moving the data
            Address::Address(ref mut next_address)=>{
                next_address.append(data);
            }
            Address::Nil=>{
                let node = my_list{
                    value:data,
                    next:Address::Nil
                };
                self.next = Address::Address(Box::new(node))
            }
        }
    }

    fn delete(&mut self, elem:i32){
        match self.next {
            Address::Address(ref mut next_address)=>{
                if next_address.value==elem{
                    println!("Deleting value {}", next_address.value);
                    self.next = next_address.next.clone()
                }else{
                    next_address.delete(elem);
                }
            }
            Address::Nil => {
                if self.value==elem{
                    self.value = 0;
                }else {
                    println!(" Could not find {elem} in linked list");
                }
            },
        }
    }
    fn count(&self)->u32{
        match self.next{
            Address::Address(ref newAddress) => 1+newAddress.count(),
            Address::Nil => 0,
        }
    }
    fn list(&self){
        if self.value==0{
            println!("The list is empty")
        }else {
            println!("{}",self.value);
            match self.next {
                Address::Address(ref next_address) => next_address.list(),
                Address::Nil => {},
            }
        }
    }

    fn update(&mut self, index:u32, elem:i32){
        
        // let mut i=0;
        // let mut j=self;
        // if i==index{
        //     j.value=elem;
        // }
        // while i<index {
        //     match  j.next {
        //         Address::Address(ref mut next_address)=>j=next_address,
        //         Address::Nil=>{}
        //     }
        //     i=i+1;
        // }
        // j.value = elem
        let mut j = self;
        // if index ==0{
        //     j.value = elem;
        // }
        // else{
            for i in 0..(index){ 
                match j.next {
                    Address::Address(ref mut next_address) =>{
                        j =  next_address;
                    },
                    Address::Nil => break,
                }
            }
            j.value  = elem;
         }

    //}
}

fn main() {
    let mut head = my_list {
        value: 8,
        next: Address::Nil,
    };
    head.append(9);
    head.append(10);
    head.append(11);
    head.list();
    println!("The size of the list is {}", head.count());
    head.update(4, 20);
    head.update(0, 6);
    head.list();
}
