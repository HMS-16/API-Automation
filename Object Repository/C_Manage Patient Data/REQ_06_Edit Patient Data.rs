<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>REQ_06_Edit Patient Data</name>
   <tag></tag>
   <elementGuidId>490ad509-b5bd-4878-a445-f18e91a6b1d2</elementGuidId>
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
  &quot;text&quot;: &quot;{\n    \&quot;name\&quot;: \&quot;${name}\&quot;,\n    \&quot;pob\&quot;: \&quot;${pob}\&quot;,\n    \&quot;dob\&quot;: \&quot;${dob}\&quot;,\n    \&quot;gender\&quot;: \&quot;${gender}\&quot;,\n    \&quot;married\&quot;: false,\n    \&quot;blood_type\&quot;: \&quot;${blood_type}\&quot;,\n    \&quot;phone_num\&quot;: \&quot;${phone_num}\&quot;,\n    \&quot;email\&quot;: \&quot;${email}\&quot;,\n    \&quot;address\&quot;: \&quot;${address}\&quot;,\n    \&quot;district\&quot;: \&quot;${district}\&quot;,\n    \&quot;city\&quot;: \&quot;${city}\&quot;,\n    \&quot;province\&quot;: \&quot;${province}\&quot;,\n    \&quot;name_family\&quot;: \&quot;${name_family}\&quot;,\n    \&quot;relationship_family\&quot;: \&quot;${relationship_family}\&quot;,\n    \&quot;phone_num_family\&quot;: \&quot;${phone_num_family}\&quot;,\n    \&quot;email_family\&quot;: \&quot;${email_family}\&quot;,\n    \&quot;address_family\&quot;: \&quot;${address_family}\&quot;,\n    \&quot;district_family\&quot;: \&quot;${district_family}\&quot;,\n    \&quot;city_family\&quot;: \&quot;${city_family}\&quot;,\n    \&quot;province_family\&quot;: \&quot;${province_family}\&quot;\n}&quot;,
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
      <webElementGuid>4304d452-ab49-43b8-9b3e-f8471c7c2d09</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${token}</value>
      <webElementGuid>7d965574-6917-4f30-acb8-d6bf40a9ec3f</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.5.5</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>PUT</restRequestMethod>
   <restUrl>https://hms-api.fly.dev/v1/patients/20042a72-a0d1-4e64-803a-f62bae30dd1b</restUrl>
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
   <variables>
      <defaultValue>'kasih aurora'</defaultValue>
      <description></description>
      <id>35a26b7b-4ae3-44ba-97d2-c78c8ef10318</id>
      <masked>false</masked>
      <name>name</name>
   </variables>
   <variables>
      <defaultValue>'tangerang'</defaultValue>
      <description></description>
      <id>24e7e9bf-2895-45e5-9dfa-e5473083b1e2</id>
      <masked>false</masked>
      <name>pob</name>
   </variables>
   <variables>
      <defaultValue>'05/13/2002'</defaultValue>
      <description></description>
      <id>9158b4ef-1544-4e34-8557-2134d3e4e1fc</id>
      <masked>false</masked>
      <name>dob</name>
   </variables>
   <variables>
      <defaultValue>'Female'</defaultValue>
      <description></description>
      <id>6cd63b0f-aeb5-461e-b6be-902700efe6f7</id>
      <masked>false</masked>
      <name>gender</name>
   </variables>
   <variables>
      <defaultValue>'A'</defaultValue>
      <description></description>
      <id>466000bc-54ee-47dd-ad11-9f83675dcc44</id>
      <masked>false</masked>
      <name>blood_type</name>
   </variables>
   <variables>
      <defaultValue>'082363236233'</defaultValue>
      <description></description>
      <id>c23c07ab-1a31-4a2f-99a3-7171c5cb0e98</id>
      <masked>false</masked>
      <name>phone_num</name>
   </variables>
   <variables>
      <defaultValue>'jl. keputih tegal bakti'</defaultValue>
      <description></description>
      <id>b22424f8-f8df-4d16-978f-771d76841eff</id>
      <masked>false</masked>
      <name>address</name>
   </variables>
   <variables>
      <defaultValue>'sukalilo'</defaultValue>
      <description></description>
      <id>9a26a62c-2f8f-4a06-b2c8-db4040d7077d</id>
      <masked>false</masked>
      <name>district</name>
   </variables>
   <variables>
      <defaultValue>'jatim'</defaultValue>
      <description></description>
      <id>64b71202-c826-41a9-b49e-3ee040383431</id>
      <masked>false</masked>
      <name>city</name>
   </variables>
   <variables>
      <defaultValue>'surabaya'</defaultValue>
      <description></description>
      <id>cc47003b-24ba-4e71-9696-41c81517cadd</id>
      <masked>false</masked>
      <name>province</name>
   </variables>
   <variables>
      <defaultValue>'Ana'</defaultValue>
      <description></description>
      <id>0094e44f-2cf6-48a1-9e03-db7a33c7592e</id>
      <masked>false</masked>
      <name>name_family</name>
   </variables>
   <variables>
      <defaultValue>'Sister'</defaultValue>
      <description></description>
      <id>7eb17aee-29fa-4baa-bc32-23348bd5951a</id>
      <masked>false</masked>
      <name>relationshipe_family</name>
   </variables>
   <variables>
      <defaultValue>'082382837733'</defaultValue>
      <description></description>
      <id>8b5867ab-7d3c-4400-8300-d43620370f0e</id>
      <masked>false</masked>
      <name>phone_num_family</name>
   </variables>
   <variables>
      <defaultValue>'ana@gmail.com'</defaultValue>
      <description></description>
      <id>d7f71e10-6b16-47dd-a7a8-97144c404afb</id>
      <masked>false</masked>
      <name>email_family</name>
   </variables>
   <variables>
      <defaultValue>'jl. hos cokroaminoto no.10'</defaultValue>
      <description></description>
      <id>faa1d306-ef12-4b47-94e1-aef2b62ef398</id>
      <masked>false</masked>
      <name>address_family</name>
   </variables>
   <variables>
      <defaultValue>'surabaya'</defaultValue>
      <description></description>
      <id>be261d44-9c04-43d1-a859-a7ea8441b4bb</id>
      <masked>false</masked>
      <name>district_family</name>
   </variables>
   <variables>
      <defaultValue>'jatim'</defaultValue>
      <description></description>
      <id>1621c9d5-feae-4328-b97a-65a5ca374f31</id>
      <masked>false</masked>
      <name>city_family</name>
   </variables>
   <variables>
      <defaultValue>'surabaya'</defaultValue>
      <description></description>
      <id>faf05b5c-0bfa-437a-b9cd-295890a32faf</id>
      <masked>false</masked>
      <name>province_family</name>
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
