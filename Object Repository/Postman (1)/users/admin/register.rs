<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>register</name>
   <tag></tag>
   <elementGuidId>2bd00115-834f-4207-a14b-f27be0761b92</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <authorizationRequest>
      <authorizationInfo>
         <entry>
            <key>bearerToken</key>
            <value>eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJleHAiOjE2NzEwNTA2ODQsImlhdCI6MTY3MTAwNzQ4NCwiaWQiOiI5ZGI0MGVmNi01N2QzLTRmYTAtYTRmMy02ZTdlNTllYTFkOGQiLCJyb2xlIjoiYWRtaW4iLCJ1c2VybmFtZSI6InRlc3RpbmcxIn0.Ldt5nZhZ7hiiJ2utN_pnCPLyRwGqhIo7Kq86COb3QaI</value>
         </entry>
      </authorizationInfo>
      <authorizationType>Bearer</authorizationType>
   </authorizationRequest>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;username\&quot;: \&quot;azhar\&quot;,\n    \&quot;password\&quot;: \&quot;azhar\&quot;,\n    \&quot;email\&quot;: \&quot;azhar@gmail.com\&quot;,\n    \&quot;role\&quot;: 1,\n    \&quot;str_num\&quot;: \&quot;2201521125678212\&quot;\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>ce900c5f-f800-46e4-89c6-24873c3bb049</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJleHAiOjE2NzEwNTA2ODQsImlhdCI6MTY3MTAwNzQ4NCwiaWQiOiI5ZGI0MGVmNi01N2QzLTRmYTAtYTRmMy02ZTdlNTllYTFkOGQiLCJyb2xlIjoiYWRtaW4iLCJ1c2VybmFtZSI6InRlc3RpbmcxIn0.Ldt5nZhZ7hiiJ2utN_pnCPLyRwGqhIo7Kq86COb3QaI</value>
      <webElementGuid>6f8539e4-c128-4c67-b1c2-fe37e471cec5</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.5.5</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://hms-api.fly.dev/v1/register</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
