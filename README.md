# Just Rust

Rust is a language that is quite new, but already very popular. It's popular because it gives you the speed and control of C or C++ but also the memory safety of other newer languages like Python. It does this with some new ideas that are sometimes different from other languages. That means that there are some new things to learn, and  you can't just "figure it out as you go along". Rust is a language that you have to think about for a while to understand. But it still looks pretty familiar if you know another language and it is made to help you write good code.

# Who am I?

I wrote Just Rust while thinking of how to make it easy for companies here to start using it.


## String

Rust has two main types of strings: `String` and `&str`. What is the difference?

* `&str` is a simple string. When you write `let my_variable = "Hello, world!"`, you create a `&str`. A `&str` is very fast.
* `String` is a more complicated string. It is a bit slower, but it has more functions. A `String` is a pointer, with data on the heap.

Also note that `&str` has the `&` in front of it because you need a reference to use a `str`. That's because of the reason we saw above: the stack needs to know the size. So we give it a `&` that it knows the size of, and then it is happy. Also, because you use a `&` to interact with a `str`, you don't own it. But a `String` is an owned type. We will soon learn why that is important to know.

```
                     buffer
                   /   capacity
                 /   /  length
               /   /   /
            +–––+–––+–––+
stack frame │ • │ 8 │ 6 │ <- my_name: String
            +–│–+–––+–––+
              │
            [–│–––––––– capacity –––––––––––]
              │
            +–V–+–––+–––+–––+–––+–––+–––+–––+
       heap │ P │ a │ s │ c │ a │ l │   │   │
            +–––+–––+–––+–––+–––+–––+–––+–––+

            [––––––– length ––––––––]
```
```
            my_name: String   last_name: &str
            [––––––––––––]    [–––––––]
            +–––+––––+––––+–––+–––+–––+
stack frame │ • │ 16 │ 13 │   │ • │ 6 │
            +–│–+––––+––––+–––+–│–+–––+
              │                 │
              │                 +–––––––––+
              │                           │
              │                           │
              │                         [–│––––––– str –––––––––]
            +–V–+–––+–––+–––+–––+–––+–––+–V–+–––+–––+–––+–––+–––+–––+–––+–––+
       heap │ P │ a │ s │ c │ a │ l │   │ P │ r │ e │ c │ h │ t │   │   │   │
            +–––+–––+–––+–––+–––+–––+–––+–––+–––+–––+–––+–––+–––+–––+–––+–––+
```
