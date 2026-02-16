// Box<T> is a smart pointer that:
// - Allocates data on the heap — stores the value in heap memory instead of the stack
// - Owns the data — Box owns the value it points to
// - Has a known size — Box itself is a pointer (typically 8 bytes on 64-bit systems), so it has a fixed size even if T doesn't

// Node struct
struct Node {
    value:i32,
    next: Option<Box<Node>>
}

// Linked list with a head pointer
struct LinkedList {
    head: Option<Box<Node>>,
}

// impl is a Rust keyword that defines an implementation block 
// for a type.
// - Define methods (functions) that operate on that type
// - Define associated functions (like constructors)
// - Implement traits for that type

impl LinkedList {
    // Associated functions (e.g., constructor)
    // Self — shorthand for LinkedList within the impl block
    fn new() -> Self {
        LinkedList { head: None }
    }

    fn push_front(&mut self, val:i32) {
        let new_node = Box::new(Node{
            value: val,
            next: self.head.take(),
        });

        self.head = Some(new_node);
    }

    fn push_back(&mut self, val:i32) {
        let mut current = &mut self.head;

        while let Some(node) = current{
            current = &mut node.next;
        }
        // Asigns to the next field a pointer
        // to the new node.
        *current = Some(Box::new(Node{
            value: val,
            next: None
        }));
    }

    fn delete(&mut self, val:i32) -> bool {
        // Special case: delete from head
        if let Some(node) = &mut self.head {
            if node.value == val {
                self.head = node.next.take();
                return true;
            }
        }

        // General case: iterate through the list
        let mut current = &mut self.head;

        while let Some(node) = current {
            // Check if the next node matches
            if let Some(next_node) = &mut node.next {
                if next_node.value == val {
                    // Remove the next node by taking its next
                    node.next = next_node.next.take();
                    return true;
                }
            }
            current = &mut node.next;
        }

        false
    }

    fn contains(&self, val:i32) -> bool {
        let mut current = &self.head;

        while let Some(node) = current {
            if node.value == val {
                return true;
            }

            current = &node.next;
        }

        false
    }

    fn print(&self) {
        let mut current = &self.head;
        
        println!("List: ");

        while let Some(node) = current {
            print!("{} -> ", node.value);
            current = &node.next;
        }
        println!("None");
    }

    fn print_header(&self) {
        let current = &self.head;
        if let Some(node) = current {
            println!("Current head: {}", node.value);
        }
    }
}

fn main() {
    let mut list = LinkedList::new();

    list.push_front(10);
    list.push_front(20);

    list.print();
    list.print_header();

    list.push_back(30);
    list.print();
    list.print_header();

    list.push_front(50);

    list.print();
    list.print_header();

    list.delete(50);

    list.print();
    list.print_header();


    println!("{}", list.contains(40));
    println!("{}", list.contains(80));

    /*list.delete(20);

    list.print();

    list.delete(40);

    list.print();

    list.delete(10);
    list.delete(30);

    list.print(); */
}
