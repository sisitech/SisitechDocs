# TEST COVERAGE

This is a metric in testing softwares which measures the amount of testing performed by a set of tests. The technique ensures your tests are testing your code . Done by the QA team

Code coverage-> unit testing practices that target all areas of your code atleast once. Performed by the developers.

## Usage of test coverage

- Finding the area of a requirement not implemented by a set of test cases
- Helps to create additional test cases to increase coverage
- Identifying a quantitative measure of test coverage, which is an indirect method for quality check
- Identifying meaningless test cases that do not increase coverage

## Test coverage techniques

**Product Coverage**- Looking at test coverage with your product in mind i.e whatareas of the product have you tested? For example:
  ***If “knife” is a product, you are testing; just do not concentrate on checking whether it cuts the vegetables/fruits properly. There are other aspects to look for such as – the user should be able to handle it comfortably***

**Risk Coverage**- listing the risks involved in the application and making sure they’re covered by tests. For Example:
  
  ***While testing an airplane, if a tester tells you that he/she has fully tested the internal system of the airplane and it’s working fine but only flying capability of the airplane was not covered while testing – what would be your reaction?***

Well, that is what risk coverage means. Identifying risk as per the application/product and testing it thoroughly is always a good practice.

**Requirements Coverage**- checks if the user's requirements are met.For Example:

***While testing a chat application, tester took care of all the important points like multiple users chatting in a group, two users chatting independently, all types of emoticons available, updates sent to user immediately etc. but forgot to look into requirement document, which clearly mentioned that when two users chat independently, video call option should be enabled.The client marketed the chat application claiming that it would allow calling, while two users chat independently. You can imagine what would have happened to the chat application.***

**Boundary Value Coverage**- Suppose you have a feature that can take numbers as input, but it should only accept numbers from 0 to  100, inclusive. In scenarios like these, to ensure the boundaries are properly tested, you’d typically test at the boundary, and immediately below and above the boundary.

**Compatibility Coverage**- make sure your tests cover your application across different browsers and operating systems

**AI-Aided Test Automation**- consists of the use of test automation tools that rely on AI to take your testing approach to a whole new level

## Performing test coverage

1. Test coverage can be done by exercising the static review techniques like peer reviews, inspections, and walkthrough
2. By transforming the ad-hoc defects into executable test cases
3. At code level or unit test level, test coverage can be achieved by availing the automated code coverage or unit test coverage tools
4. Functional test coverage can be done with the help of proper test management tools

## Advantages

- It can assure the quality of the test
- It can help identify what portions of the code were actually touched for the release or fix
- It can help to determine the paths in your application that were not tested
- Prevent Defect leakage
- Time, scope and cost can be kept under control
- Defect prevention at an early stage of the project lifecycle
- It can determine all the decision points and paths used in the application, which allows you to increase test coverage
- Gaps in requirements, test cases and defects at the unit level and code level can be found in an easy way

## Disadvantages

- Most of the tasks in the test coverage manual as there are no tools to automate. Therefore, it takes lots of effort to  analyze the requirements and create test cases.
- Test coverage allows you to count features and then measure against several tests. However, there is always space for judgment errors.

## Differences between tests coverage and code coverage

| Code coverage                                                                                                                      | Test Coverage                                                                                                |
|------------------------------------------------------------------------------------------------------------------------------------|--------------------------------------------------------------------------------------------------------------|
| Code coverage term used when application code is exercised when an application is running                                          | Test coverage means overall test-plan.                                                                       |
| Code coverage metrics can help the team monitor their automated tests                                                              | Test coverage is given details about the level to which the written coding of an application has been tested |
| Code coverage is divided with subtypes like statement coverage, condition coverage, Branch coverage, Toogle coverage, FSM coverage | No subtype of Test coverage method                                                                           |

## References

1. https://www.guru99.com/test-coverage-in-software-testing.html
2. https://www.softwaretestinghelp.com/test-coverage/
3. https://www.testim.io/blog/test-coverage-techniques/

### Video References

- https://www.youtube.com/watch?v=cMMXvozoXBk
- https://www.youtube.com/watch?v=-MKSYGpxG5g