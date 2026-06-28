# Calscin

Calscin is a programming language focusing on allowing everyone to build reliable, efficient and safe software. 

---

![Codacy grade](https://img.shields.io/codacy/grade/954560aa46bb4487815c1c7511ac7630?style=for-the-badge) ![Deps.rs Repository Dependencies](https://img.shields.io/deps-rs/repo/github/calscin/calscin?style=for-the-badge)
---

> [!IMPORTANT]  
> Calscin is currently in heavy development and might be not ready to use in production.

## Why Calscin?
---
- **Performance**: Calscin focuses on being fast, suitable for services where performance is critical and where resources are limited.
- **Freedom**: Calscin tries to provide more freedom to the developper to help them create programs tailored to their needs. 
- **Adaptability**: Calscin is designed to make both low level hardware projects and high level programs work by providing tools and the ability to modify them!

## Quick Example
```calscin
// This is a structure: a type containing fields
prot struct myStruct {
	s.32 test
}

module myModule {
	module myInnerModule {
		pub struct myPublicStructure {
			str myText
		}
	}
}

func main() {
	// Since the variable was declared using the var keyword, it is immutable!
	var myStruct myStructuredVariable = { test: 3 };

	// Since the variable was declared using the mut keyword, it is mutable!
	mut str myString = "Hello World";

	myString = "Hello Calscin!"; 
	
	var myModule::myInnerModule::myPublicStructure myStructure = { myText: myString };
}

```
