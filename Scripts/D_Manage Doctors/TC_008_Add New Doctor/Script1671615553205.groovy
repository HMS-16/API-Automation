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

def response = WS.sendRequest(findTestObject('D_Manage Doctors/REQ_08_Add New Doctor', [('token') : GlobalVariable.tokenAdminWeb
            , ('str_num') : '3243242343221111', ('name') : 'Agus', ('pob') : 'kediri', ('dob') : '05/13/2002', ('gender') : '"Male"'
            , ('phone_num') : '08129123923189', ('email') : 'agus@gmail.com', ('last_education') : 'unairr', ('competency') : 'tooth'
            , ('address') : 'jl. keputih tegal bakti', ('district') : 'sukolilo', ('city') : 'surabaya', ('province') : 'jawa timur']))

def datafile = findTestData('Data Files/D_Manage Doctors/Add New Doctor Valid')

for (def Row = 1; Row <= datafile.getRowNumbers(); Row++) {
    WS.verifyResponseStatusCode(response, 200)
}

