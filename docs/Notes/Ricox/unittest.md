# Software Testing

This is the execution of a software system to assure that it is bug-free.

### Categories of software testing
- Functional Testing
- Non-Functional Testing

### Functional Testing
It reviews every aspect of a piece of software to make sure it is working correctly.

#### Types of Functional Testing
- Unit Testing
- Intergration Testing
- Smoke Testing - works with the major functionalities
- Sanity Testing - additional functionalities and minor changes
- Regression Testing - verify additions and bug fixes do not affect existing features 
- User Acceptance Testing - based on users comfort

Functional Testing Adoption includes: 
- Black Box Testing - functionalityis tested without looking at design/structure
- White Box Testing- focuses on the robustness of code rather than the functionality (internal functionality is known to  the tester
- Grey Box Testing - development moves into crossover zone towards the end of intergrstion and beginnig of usability

### Non Functional testing
Checks for the behaviour of the application
Done based on customer expectation and performance requirement.

#### Types of Non Functional testing
- Performance Testing
- Load testing
- Stress testing
- Volume testing
- Security testing
- Installation testing
- Recovery testing

----
#### **Unit Testing**

Tests an individual unit of software to validate that each unit performs as designed in isolation

Done by Developers

**Component Testing** is done by Testers
Type of *white box* testing

#### Best practices

- Write comprehensive, independent test cases 
- Write test cases before fixing defects
- Execute test cases continuously and frequently



#### Intergration Testing

Takes multiple units and tests them as a group to ensure that the modules connect as expected
Done by *tester*
Type of *black box* testing
Deals with interfaces


#### Types of Intergration testing
1. **Big Bang Approach**

Intergeates all modules in one go

If an issue is detected it becomes difficult to find out the module that caused the issue.
A good approach for small systems

![big-bang](https://www.softwaretestinghelp.com/wp-content/qa/uploads/2016/12/integration-testing_big-bang-approach.png)

#### Intergration testing steps
Prepare Intergration
	- Test plan
	- Test Scenarios / cases
	- test automation scripts
Execute test cases
Report defects
Track and re-test defects
Retest until intergration testing is complete

#### Test Intergration Approaches

1. Bottom-up approach
2. Top-down approach


#### *Bottom-up approach*

units are tested from bottom to top until all level of units are intergrated and tested as one unit
*Drivers* are used in this approach - dummy programs used to call functions of lowest module in a case when the calling function does not exist.
It is easier to detect issues on lower levels making high-level issues be identified at end when all units have been intergrated.

![bottom-up](https://www.softwaretestinghelp.com/wp-content/qa/uploads/2016/12/integration-testing_bottom-up.jpeg)

#### *Top-down approach*

units are tested from top to bottom levels step by step

first unit - tested individually by writing  *test STUBS*
lower levels intergrated one by one unitl the last level is reached
It provides consistency in relation to how things happen in the real environment
Major functionality is tested at end.

![top-down](https://www.softwaretestinghelp.com/wp-content/qa/uploads/2016/12/integration-testing_top-down-approach.jpeg)

----
### UI TESTING
It involves testing all visual indicators and graphical icons, including menus, radio buttons, text boxes, checkboxes, toolbars, colors, fonts, and more.

The main aspects checked in UI testing include:

- Visual Design
- Functionality
- Usability
- Performance
- Compliance

**Approaches to UI testing**
1. ### Manual Testing
Tested by a human

1. ### Record-and Playback Testing
Tested using automated tools

1. ### Model-Based Testing
Model-based testing works as follows:

 -   Create a model for the system
  -  Determine system inputs
  -  Verify the expected output
   - Execute tests
   - Check and validate system output vs. the expected output

The model-based approach is great because it allows a higher level of automation. 

Demo practical test


![google test case](https://www.perfecto.io/sites/default/files/image/2021-01/image2-2.png)

Using the above form, we identify 13 test cases, labeled TC-1 to TC-13. At the very least, we should perform the following UI checks:

TC-1

    - Check the page label, position, and font.

TC-2

   - Validate whether the page heading is correct.
   - Check the font used.

TC-3

   - Check the cursor focus on the default field.
   - Test the mandatory fields by clicking next while the form is blank.
  -  Check the position and alignment of the text box.
   - Check the acceptance of both valid and invalid characters in the field labels.

TC-4

   - Check the position and alignment of the text box.
   - Check field labels, validate the acceptance of both valid and invalid characters.

TC-5

   - Check the position and alignment of the text box.
   - Check field labels, validate the acceptance of both valid and invalid characters.

TC-6

- Test the error message by entering both permitted and prohibited characters.
  -  Verify error message correctness.

TC-7

  -  Test pop-ups and hyperlinks.

TC-8

  -  Check field labels, validate the acceptance of both valid and invalid characters.
 -   Check the position and alignment of the text box.

TC-9

 -   Save an unmatched password.
-  Check field labels, validate the acceptance of both valid and invalid characters.
 - Check the position and alignment of the text box.

TC-10

  -  Verify icon position.
  -  Test the icon shows or hides the user password.
 -   Check the image quality.

TC-11

 -   Test the error message by entering both permitted and prohibited characters.
  -  Verify error message correctness.

TC-12

  -  Test pop-ups and hyperlinks.

TC-13

  -  Test form submission.
  -  Check button position and clarity.



### REFERENCES
1. https://www.perfecto.io/blog/ui-testing-comprehensive-guide
1. https://www.softwaretestinghelp.com/what-is-integration-testing/
1. https://www.softwaretestinghelp.com/the-difference-between-unit-integration-and-functional-testing/
