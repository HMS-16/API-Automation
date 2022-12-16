 Feature: Sign in as nurse Feature
  @positiveasNurse
  Scenario Outline: SignIn as Doctor with valid data
    Given Post call to url Sign In as Doctor with valid data and the nurse successfully sign in
    Then verify the response body and status code is 200 for nurse

  @negativeAsNurse
  Scenario Outline: SignIn as Nurse with invalid data
    Given Post call to url Sign In as Nurse with empty email and empty password
    Then verify the response body and status code is 400 for nurse

  @negativeAsNurse
  Scenario Outline: SignIn as Nurse with invalid data
    Given PPost call to url Sign In as Nurse with empty email
    Then verify the response body and status code is 400 for nurse

  @negativeAsNurse
  Scenario Outline: SignIn as Nurse with invalid data
    Given PPost call to url Sign In as Nurse withand empty password
    Then verify the response body and status code is 400 for nurse