 Feature: Sign in as nurse Feature
 @positiveasDoctor
  Scenario Outline: SignIn as Doctor with valid data
    Given Post call to url Sign In as doctor with valid data
    Then verify the response body and status code is 200 and the doctor successfully sign in

  @negativeAsDoctor
  Scenario Outline: SignIn as Doctor with invalid data
    Given Post call to url Sign In as Doctor with empty email and empty password
    Then verify the response body and status code is 400 for doctor

  @negativeAsDoctor
  Scenario Outline: SignIn as Doctor with invalid data
    Given Post call to url Sign In as Doctor with empty email
    Then verify the response body and status code is 400 for doctor

  @negativeAsDoctor
  Scenario Outline: SignIn as Doctor with invalid data
    Given Post call to url Sign In as Doctor with empty password
    Then verify the response body and status code is 400 for doctor
