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

response = WS.sendRequest(findTestObject('USERS/POST - users'))

WS.verifyResponseStatusCode(response, 201, FailureHandling.STOP_ON_FAILURE)

assert response.getStatusCode() == 201

WS.verifyElementPropertyValue(response, 'name', 'akun test')

WS.verifyElementPropertyValue(response, 'username', 'akuntest')

WS.verifyElementPropertyValue(response, 'email', 'akuntest@gmail.com')

WS.verifyElementPropertyValue(response, 'address.street', 'street boba')

WS.verifyElementPropertyValue(response, 'address.suite', 'Apt. 556')

WS.verifyElementPropertyValue(response, 'address.city', 'sorolangun')

WS.verifyElementPropertyValue(response, 'address.zipcode', '48394')

WS.verifyElementPropertyValue(response, 'address.geo.lat', '-37.3159')

WS.verifyElementPropertyValue(response, 'address.geo.lng', '81.1496')

WS.verifyElementPropertyValue(response, 'phone', '59848696656')

WS.verifyElementPropertyValue(response, 'website', 'akuntest.org')

WS.verifyElementPropertyValue(response, 'company.name', 'akun test')

WS.verifyElementPropertyValue(response, 'company.catchPhrase', 'multi')

WS.verifyElementPropertyValue(response, 'company.bs', 'e-markets')

