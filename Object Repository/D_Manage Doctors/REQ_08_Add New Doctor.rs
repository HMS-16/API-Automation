<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>REQ_08_Add New Doctor</name>
   <tag></tag>
   <elementGuidId>1b661627-18fa-4870-b075-467c702fb213</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <authorizationRequest>
      <authorizationInfo>
         <entry>
            <key>bearerToken</key>
            <value>${token}</value>
         </entry>
      </authorizationInfo>
      <authorizationType>Bearer</authorizationType>
   </authorizationRequest>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;str_num\&quot;: \&quot;1423242341325678\&quot;,\n    \&quot;name\&quot;: \&quot;shaffa\&quot;,\n    \&quot;pob\&quot;: \&quot;kediri\&quot;,\n    \&quot;dob\&quot;: \&quot;02/15/2000\&quot;,\n    \&quot;gender\&quot;: \&quot;Male\&quot;,\n    \&quot;married\&quot;: false,\n    \&quot;phone_num\&quot;: \&quot;021312412312\&quot;,\n    \&quot;email\&quot;: \&quot;azhar@gmail.com\&quot;,\n    \&quot;last_education\&quot;: \&quot;universitas airlangga\&quot;,\n    \&quot;graduation_year\&quot;: 2020,\n    \&quot;exp_year\&quot;: 2,\n    \&quot;competency\&quot;: \&quot;Dermatologi\&quot;,\n    \&quot;address\&quot;: \&quot;j;. keputih tegal bakti\&quot;,\n    \&quot;district\&quot;: \&quot;sukolilo\&quot;,\n    \&quot;city\&quot;: \&quot;surabaya\&quot;,\n    \&quot;province\&quot;: \&quot;jawa timur\&quot;\n}&quot;,
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
      <webElementGuid>ee6e2b68-9f29-426e-92b4-06318e82efdb</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${token}</value>
      <webElementGuid>88a1133c-bff2-47e6-9c32-cb45f317f895</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.5.5</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://hms-api.fly.dev/v1/doctors</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.tokenAdminWeb</defaultValue>
      <description></description>
      <id>f28d314b-2bec-44b9-8536-f59154ce8f2e</id>
      <masked>false</masked>
      <name>token</name>
   </variables>
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
