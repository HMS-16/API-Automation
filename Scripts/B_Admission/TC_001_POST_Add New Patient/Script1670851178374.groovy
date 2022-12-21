import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys


def response = WS.sendRequest(findTestObject('B_Admisson/REQ_04_Add New Patient_Website', [('token') : GlobalVariable.tokenAdminWeb, ('name') : 'kasih auroras'
            , ('pob') : 'tangerangs', ('dob') : '05/13/2002', ('gender') : 'Female', ('blood_type') : 'A', ('phone_num') : '082363236232'
            , ('address') : 'jl. keputih tegal baktis', ('district') : 'sukalilos', ('city') : 'jatims', ('province') : 'surabayas'
            , ('name_family') : 'Anas', ('relationshipe_family') : 'Sister', ('phone_num_family') : '0823828377332', ('email_family') : 'ana2@gmail.com'
            , ('address_family') : 'jl. hos cokroaminoto no.11', ('district_family') : 'surabayaw', ('city_family') : 'jatims'
            , ('province_family') : 'surabaya']))

def datafile = findTestData('Data Files/B_Admission/Add Patient Data_Valid')

for (def Row = 1; Row <= datafile.getRowNumbers(); Row++) {
    WS.verifyResponseStatusCode(response, 200)
}

