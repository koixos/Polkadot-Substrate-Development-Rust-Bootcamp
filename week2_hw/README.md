# Polkadot Rust Bootcamp - HW 1

### Task Details

In this task, a simple Rust program to be created which demonstrates the concepts of ownership, borrowing, and references. The program will take two strings as input, concatenate them, and then print the result without violating any ownership rules.

---
### Steps

1. Create a function called *concatenate_strings* that takes two string slices as arguments and returns a new `String` as the result of concatenating the two input strings.

2. Inside the *concatenate_strings* function, create a new `String` called *result*. Use the `push_str()` method to append the contents of the first input string slice, followed by the second input string slice.

3. Return the result string from the function.

4. In the `main` function, create two `String` variables, *string1* and *string2*, and initialize them with appropriate values.

5. Call the *concatenate_strings* function with references to *string1* and *string2* as arguments (**using string slices**). Store the result in a new variable called *concatenated_string*.

6. Print the *concatenated_string* variable to the console.

7. Compile and run the program to ensure it works as expected.

---
### Checklist

* Write the *concatenate_strings* function signature.

* Implement the *concatenate_strings* function.

* Initialize two `String` variables in the `main` function.

* Call the *concatenate_strings* function with string slices of the variables.

* Print the *result* to the console.

* Compile and run the program to test its functionality.
