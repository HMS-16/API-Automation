Feature: Sign in Feature

  @positiveAsAdmin
  Scenario Outline: SignIn as Admin with invalid data
    Given Post call to url Sign In as admin with valid data
    Then verify the response body and status code is 200 and the admin successfully sign in

  @negativeAsAdmin
  Scenario Outline: SignIn as Admin with invalid data
    Given Post call to url Sign In as admin with empty username and empty password
    Then verify the response body and status code is 400 for admin

  @negativeasAdmin
  Scenario Outline: SignIn as Admin with invalid data
    Given Post call to url Sign In as admin with empty username
    Then verify the response body and status code is 400 for admin

  @negativeasAdmin
  Scenario Outline: SignIn as Admin with invalid data
    Given Post call to url Sign In as admin with  empty password
    Then verify the response body and status code is 400 for admin

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

  @positiveasNurse
  Scenario Outline: SignIn as Nurse with valid data
    Given Post call to url Sign In as Nurse with valid data and the nurse successfully sign in
    Then verify the response body and status code is 200 for nurse

  @negativeAsNurse
  Scenario Outline: SignIn as Nurse with invalid data
    Given Post call to url Sign In as Nurse with empty email and empty password
    Then verify the response body and status code is 400 for nurse

  @negativeAsNurse
  Scenario Outline: SignIn as Nurse with invalid data
    Given Post call to url Sign In as Nurse with empty email
    Then verify the response body and status code is 400 for nurse

  @negativeAsNurse
  Scenario Outline: SignIn as Nurse with invalid data
    Given Post call to url Sign In as Nurse withand empty password
    Then verify the response body and status code is 400 for nurse
