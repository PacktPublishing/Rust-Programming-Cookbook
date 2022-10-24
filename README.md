# Rust Programming Cookbook 

<a href="https://www.packtpub.com/programming/rust-programming-cookbook?utm_source=github&utm_medium=repository&utm_campaign=9781789530667"><img src="https://www.packtpub.com/media/catalog/product/cache/e4d64343b1bc593f1c5348fe05efa4a6/9/7/9781789530667-original.jpeg" alt="Rust Programming Cookbook " height="256px" align="right"></a>

This is the code repository for [Rust Programming Cookbook ](https://www.packtpub.com/programming/rust-programming-cookbook?utm_source=github&utm_medium=repository&utm_campaign=9781789530667), published by Packt.

**Explore the latest features of Rust 2018 for building fast and secure apps**

## What is this book about?
Rust 2018, Rust's first major milestone since version 1.0, brings more advancement in the Rust language. The Rust Programming Cookbook is a practical guide to help you overcome challenges when writing Rust code.


This book covers the following exciting features:
Understand how Rust provides unique solutions to solve system programming language problems 
Grasp the core concepts of Rust to develop fast and safe applications 
Explore the possibility of integrating Rust units into existing applications for improved efficiency 
Discover how to achieve better parallelism and security with Rust 
Write Python extensions in Rust 
Compile external assembly files and use the Foreign Function Interface (FFI) 
Build web applications and services using Rust for high performance 

If you feel this book is for you, get your [copy](https://www.amazon.com/dp/1789530660) today!

<a href="https://www.packtpub.com/?utm_source=github&utm_medium=banner&utm_campaign=GitHubBanner"><img src="https://raw.githubusercontent.com/PacktPublishing/GitHub/master/GitHub.png" 
alt="https://www.packtpub.com/" border="5" /></a>

## Instructions and Navigations
All of the code is organized into folders. For example, Chapter02.

The code will look like the following:
```
fn main() -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
   let mut ls_child = Command::new("ls");
   if !cfg!(target_os = "windows") {
       ls_child.args(&["-alh"]);
```

**Following is what you need for this book:**
The Rust cookbook is for software developers looking to enhance their knowledge of Rust and leverage its features using modern programming practices. Familiarity with Rust language is expected to get the most out of this book.


With the following software and hardware list you can run all code files present in the book (Chapter 1-).
### Software and Hardware List
| No | Software required | OS required |
| -------- | ------------------------------------ | ----------------------------------- |
| 1 | Rust nightly > 2019-10-10 | Windows/Linux/macOS |
| 2 | Visual Studio Code | Windows/Linux/macOS |
| 3 | Docker CE stable 19.03 | Windows/Linux/macOS |
| 4 | Node.js 10.16.3 | Windows/Linux/macOS |
| 5 | Python 3.6 or later | Windows/Linux/macOS |
| 6 | gcc >= 9.2 | Windows/Linux/macOS |
| 7 | Rust stable > 1.38.0 | Windows/Linux/macOS |

We also provide a PDF file that has color images of the screenshots/diagrams used in this book. [Click here to download it](https://static.packt-cdn.com/downloads/9781789530667_ColorImages.pdf).

### Related products
* Mastering Rust - Second Edition  [[Packt]](https://www.packtpub.com/application-development/mastering-rust-second-edition?utm_source=github&utm_medium=repository&utm_campaign=9781789346572) [[Amazon]](https://www.amazon.com/dp/B07GVNJ77X)

* Hands-On Data Structures and Algorithms with Rust  [[Packt]](https://www.packtpub.com/application-development/hands-data-structures-and-algorithms-rust?utm_source=github&utm_medium=repository&utm_campaign=9781788995528) [[Amazon]](https://www.amazon.com/dp/178899552X)

## Get to Know the Author
**Claus Matzinger**
is a software engineer with a very diverse background. After working in a small company maintaining code for embedded devices, he joined a large corporation to work on legacy Smalltalk applications. This led to a great interest in programming languages early on, and Claus became the CTO for a health games start-up based on Scala technology. Since then, Claus' roles have shifted toward customer-facing roles in the IoT database technology start-up, Crate IO (creators of CrateDB), and, most recently, Microsoft. There, he hosts a podcast, writes code together with customers, and blogs about the solutions arising from these engagements. For more than 5 years, Claus has been implementing software to help customers innovate, achieve, and maintain success.

### Suggestions and Feedback
[Click here](https://docs.google.com/forms/d/e/1FAIpQLSdy7dATC6QmEL81FIUuymZ0Wy9vH1jHkvpY57OiMeKGqib_Ow/viewform) if you have any feedback or suggestions.


### Special thanks 

Rust is an evolving language and we rely on you to point out anything that has changed since the book release. Open an issue if you find something won't compile and we'll gladly help out.

@tomus85 - Thank you updating three recipes: static-web, json-handling, and legacy-c-code.### Download a free PDF

 <i>If you have already purchased a print or Kindle version of this book, you can get a DRM-free PDF version at no cost.<br>Simply click on the link to claim your free PDF.</i>
<p align="center"> <a href="https://packt.link/free-ebook/9781789530667">https://packt.link/free-ebook/9781789530667 </a> </p>