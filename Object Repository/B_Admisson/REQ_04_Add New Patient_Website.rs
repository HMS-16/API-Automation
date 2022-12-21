<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>REQ_04_Add New Patient_Website</name>
   <tag></tag>
   <elementGuidId>41cbb311-80a8-45c7-86ff-996756314399</elementGuidId>
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
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;name\&quot;: \&quot;azhar\&quot;,\n    \&quot;pob\&quot;: \&quot;kediri\&quot;,\n    \&quot;dob\&quot;: \&quot;05/13/2002\&quot;,\n    \&quot;gender\&quot;: \&quot;Male\&quot;,\n    \&quot;married\&quot;: false,\n    \&quot;blood_type\&quot;: \&quot;0\&quot;,\n    \&quot;phone_num\&quot;: \&quot;08129123923189\&quot;,\n    \&quot;email\&quot;: \&quot;azhar@gmail.com\&quot;,\n    \&quot;address\&quot;: \&quot;jl. keputih tegal bakti\&quot;,\n    \&quot;district\&quot;: \&quot;sukolilo\&quot;,\n    \&quot;city\&quot;: \&quot;surabaya\&quot;,\n    \&quot;province\&quot;: \&quot;jawa timur\&quot;,\n    \&quot;name_family\&quot;: \&quot;linda\&quot;,\n    \&quot;relationship_family\&quot;: \&quot;bestie\&quot;,\n    \&quot;phone_num_family\&quot;: \&quot;08123123123\&quot;,\n    \&quot;email_family\&quot;: \&quot;linda@gmail.com\&quot;,\n    \&quot;address_family\&quot;: \&quot;jl. hos cokroaminoto no.10\&quot;,\n    \&quot;district_family\&quot;: \&quot;santren\&quot;,\n    \&quot;city_family\&quot;: \&quot;kediri\&quot;,\n    \&quot;province_family\&quot;: \&quot;jawa timur\&quot;\n}&quot;,
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
      <webElementGuid>dbc7f8b4-35fe-4066-84c4-4d4dc771236c</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${token}</value>
      <webElementGuid>aa6a4309-4a85-41cf-a7fc-c7db14f4d410</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.5.5</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://hms-api.fly.dev/v1/patients</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
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