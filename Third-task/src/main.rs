struct Student {
    id: u32,
    name: String,
    class: Class,
    clubs: Vec<Club>
  }
  
  struct Club {
    name: String,
    members: Vec<Student>
  }
  
  struct Class {
    name: String,
    students: Vec<Student>
  }
  
  struct Course {
    name: String,
    students: Vec<Student>
  }
  
  struct School {
    students: Vec<Student>,
    classes: Vec<Class>,
    clubs: Vec<Club>,
    courses: Vec<Course>
  }
  
  impl School {
  
    fn add_student(&mut self, student: Student) {
      self.students.push(student);
    }
  
    fn find_student(&self, id: u32) -> Option<&mut Student> {
      self.students.iter_mut().find(|s| s.id == id)
    }
  
    fn update_student(&mut self, id: u32, name: String) {
      if let Some(student) = self.find_student(id) {
        student.name = name;
      }
    }
  
    fn remove_student(&mut self, id: u32) {
      if let Some(student) = self.find_student(id) {
        self.students.remove_element(&mut student);
      }
    }
  }
  
  fn main() {
  
    let mut school = School {
      students: Vec::new(),
      classes: Vec::new(),
      clubs: Vec::new(),
      courses: Vec::new()
    };
  
    let student = Student {
      id: 1,
      name: "张三".to_string(),
      class: Class::new("三年级二班"),
      clubs: Vec::new()
    };
  
    school.add_student(student);
  
    if let Some(st) = school.find_student(1) {
      println!("Found student: {}", st);
    }
  
    school.update_student(1, "李四".to_string());
  
    school.remove_student(1);
  
  }
  