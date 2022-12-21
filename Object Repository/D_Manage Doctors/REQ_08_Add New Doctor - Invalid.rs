<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>REQ_08_Add New Doctor - Invalid</name>
   <tag></tag>
   <elementGuidId>60666df9-bbbd-4989-978b-371cc7cd0cb0</elementGuidId>
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
  &quot;text&quot;: &quot;{\n    \&quot;str_num\&quot;: \&quot;${str_num}\&quot;,\n    \&quot;name\&quot;: \&quot;${name}\&quot;,\n    \&quot;pob\&quot;: \&quot;${pob}\&quot;,\n    \&quot;dob\&quot;: \&quot;${dob}\&quot;,\n    \&quot;gender\&quot;: \&quot;${gender}\&quot;,\n    \&quot;married\&quot;: false,\n    \&quot;phone_num\&quot;: \&quot;${phone_num}\&quot;,\n    \&quot;email\&quot;: \&quot;${email}\&quot;,\n    \&quot;last_education\&quot;: \&quot;${last_education}\&quot;,\n    \&quot;graduation_year\&quot;: 2020,\n    \&quot;exp_year\&quot;: 2,\n    \&quot;competency\&quot;: \&quot;${competency}\&quot;,\n    \&quot;address\&quot;: \&quot;${address}\&quot;,\n    \&quot;district\&quot;: \&quot;${district}\&quot;,\n    \&quot;city\&quot;: \&quot;${city}\&quot;,\n    \&quot;province\&quot;: \&quot;${province}\&quot;\n}&quot;,
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
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>8696dda7-c648-48e9-9834-63d54940be36</id>
      <masked>false</masked>
      <name>str_num</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>5181f1d0-c3e7-4e10-9477-660a3a5410c6</id>
      <masked>false</masked>
      <name>name</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>6c82dc4a-d056-427c-9e33-ba48c283a968</id>
      <masked>false</masked>
      <name>pob</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>ed35d86a-69e8-4c2a-9ad5-fc3b7620e943</id>
      <masked>false</masked>
      <name>dob</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>852ea17a-10d2-4373-80bb-c64b7f682563</id>
      <masked>false</masked>
      <name>gender</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>0c5a6bd0-857a-4fc4-9da4-da146ecd5f28</id>
      <masked>false</masked>
      <name>phone_num</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>3f84e8a3-3795-4918-bf93-9fa7f03587ef</id>
      <masked>false</masked>
      <name>email</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>d247afd6-11d0-4d84-87b5-f14582691a56</id>
      <masked>false</masked>
      <name>last_education</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>43b80e9a-1f55-460f-a5e0-89fe9dd93c17</id>
      <masked>false</masked>
      <name>competency</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>9f038849-354e-4d16-a6ae-5e094c84cde7</id>
      <masked>false</masked>
      <name>address</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>56292780-def8-47bf-89bf-44fc3fbdf3de</id>
      <masked>false</masked>
      <name>district</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>19fa4dbc-4b57-4af0-b5ab-77717f77f1e6</id>
      <masked>false</masked>
      <name>city</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>5de9dca8-edba-4310-9496-58953c7d6c4d</id>
      <masked>false</masked>
      <name>province</name>
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
