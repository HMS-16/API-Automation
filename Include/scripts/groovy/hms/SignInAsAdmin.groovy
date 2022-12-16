package hms
import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject

import com.kms.katalon.core.annotation.Keyword
import com.kms.katalon.core.checkpoint.Checkpoint
import com.kms.katalon.core.checkpoint.CheckpointFactory
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling
import com.kms.katalon.core.testcase.TestCase
import com.kms.katalon.core.testcase.TestCaseFactory
import com.kms.katalon.core.testdata.TestData
import com.kms.katalon.core.testdata.TestDataFactory
import com.kms.katalon.core.testobject.ObjectRepository
import com.kms.katalon.core.testobject.TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI

import internal.GlobalVariable

import org.openqa.selenium.WebElement
import org.openqa.selenium.WebDriver
import org.openqa.selenium.By

import com.kms.katalon.core.mobile.keyword.internal.MobileDriverFactory
import com.kms.katalon.core.webui.driver.DriverFactory

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.testobject.ConditionType
import com.kms.katalon.core.testobject.TestObjectProperty

import com.kms.katalon.core.mobile.helper.MobileElementCommonHelper
import com.kms.katalon.core.util.KeywordUtil

import com.kms.katalon.core.webui.exception.WebElementNotFoundException

import cucumber.api.java.en.And
import cucumber.api.java.en.Given
import cucumber.api.java.en.Then
import cucumber.api.java.en.When



class SignInAsAdmin {
	@Given("Post call to url Sign In as admin with valid data")
	def postValidSignInAsAdmin() {
//		statusCode = WS.sendRequest(findTestObject('Request/Authentication/Login', [('BASE_URL') : GlobalVariable.BASE_URL]))
	}

	@Then("verify the response body and status code is 200 and the admin successfully sign in")
	def responsevalidSignInAsAdmin() {
//		WS.verifyResponseStatusCode(statusCode, 200)
//
//		WS.verifyElementPropertyValue(statusCode, 'data', "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJGdWxsbmFtZSI6IkZpcnN0bmFtZSBMYXN0bmFtZSIsIkVtYWlsIjoic29tZW9uZUBtYWlsLmNvbSJ9.qWDOcd2UDoTGumbak_fRlZZtf-H7va3sH2a339KGiKU")
	}
	@Given("Post call to url Sign In as admin with empty username and empty password")
	def postSigninEmptyAsAdmin() {
//		statusCode = WS.sendRequest(findTestObject('Request/Authentication/Login_empty data'))
	}
	
	@Given("Post call to url Sign In as admin with empty username")
	def postSigninEmptyUsernameAsAdmin() {
//		statusCode = WS.sendRequest(findTestObject('Request/Authentication/Login_empty data'))
	}
	
	@Given("Post call to url Sign In as admin with  empty password")
	def postSigninEmptyPasswordAsAdmin() {
//		statusCode = WS.sendRequest(findTestObject('Request/Authentication/Login_empty data'))
	}
	
	@Then("verify the response body and status code is 400 for admin")
	def responseSigninEmptyAsAdmin() {
//		WS.verifyResponseStatusCode(statusCode, 400)
	}

}