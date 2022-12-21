import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import internal.GlobalVariable as GlobalVariable

def response = WS.sendRequest(findTestObject('C_Manage Patient Data/REQ_06_Edit Patient Data', [('token') : GlobalVariable.tokenAdminWeb
            , ('name') : '', ('pob') : '', ('dob') : '', ('gender') : '', ('blood_type') : '', ('phone_num') : '', ('address') : ''
            , ('district') : '', ('city') : '', ('province') : '', ('name_family') : '', ('relationshipe_family') : '', ('phone_num_family') : ''
            , ('email_family') : '', ('address_family') : '', ('district_family') : '', ('city_family') : '', ('province_family') : '']))

def datafile = findTestData('Data Files/C_Manage Patient Data/Edit Patient Invalid')

for (def Row = 1; Row <= datafile.getRowNumbers(); Row++) {
    WS.verifyResponseStatusCode(response, 400)
}

