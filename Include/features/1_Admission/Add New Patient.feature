Feature: Add New Patient Feature

  @positive
  Scenario Outline: Add new patient with valid data
    Given I set endpoint to add new patient
    When I set request body to add new patients
    Then verify response valid create order

  @negativeEmptyName
  Scenario Outline: Create order Feature with empty data
    Given I set endpoint to add new patient
    When I set request body that have empty Name
    Then verify response invalid add patient
    
   @negativeEmptyGender
  Scenario Outline: Create order Feature with empty data
    Given I set endpoint to add new patient
    When I set request body that have empty Name
    Then verify response invalid add patient
  

