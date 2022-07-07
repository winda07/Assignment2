<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>GET - posts</name>
   <tag></tag>
   <elementGuidId>c1b8b830-84ab-41ef-a310-2b09ba04fe37</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <katalonVersion>8.3.5</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>http://jsonplaceholder.typicode.com/posts</restUrl>
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

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()


WS.verifyResponseStatusCode(response, 200)

assertThat(response.getStatusCode()).isEqualTo(200)


WS.verifyElementPropertyValue(response, '[0].userId', '1')
WS.verifyElementPropertyValue(response, '[0].id', '1')
WS.verifyElementPropertyValue(response, '[0].title', 'sunt aut facere repellat provident occaecati excepturi optio reprehenderit')
WS.verifyElementPropertyValue(response, '[0].body', 'quia et suscipit\nsuscipit recusandae consequuntur expedita et cum\nreprehenderit molestiae ut ut quas totam\nnostrum rerum est autem sunt rem eveniet architecto')


WS.verifyElementPropertyValue(response, '[1].userId', '1')
WS.verifyElementPropertyValue(response, '[1].id', '2')
WS.verifyElementPropertyValue(response, '[1].title', 'qui est esse')
WS.verifyElementPropertyValue(response, '[1].body', 'est rerum tempore vitae\nsequi sint nihil reprehenderit dolor beatae ea dolores neque\nfugiat blanditiis voluptate porro vel nihil molestiae ut reiciendis\nqui aperiam non debitis possimus qui neque nisi nulla')


WS.verifyElementPropertyValue(response, '[2].userId', '1')
WS.verifyElementPropertyValue(response, '[2].id', '3')
WS.verifyElementPropertyValue(response, '[2].title', 'ea molestias quasi exercitationem repellat qui ipsa sit aut')
WS.verifyElementPropertyValue(response, '[2].body', 'et iusto sed quo iure\nvoluptatem occaecati omnis eligendi aut ad\nvoluptatem doloribus vel accusantium quis pariatur\nmolestiae porro eius odio et labore et velit aut')


WS.verifyElementPropertyValue(response, '[3].userId', '1')
WS.verifyElementPropertyValue(response, '[3].id', '4')
WS.verifyElementPropertyValue(response, '[3].title', 'eum et est occaecati')
WS.verifyElementPropertyValue(response, '[3].body', 'ullam et saepe reiciendis voluptatem adipisci\nsit amet autem assumenda provident rerum culpa\nquis hic commodi nesciunt rem tenetur doloremque ipsam iure\nquis sunt voluptatem rerum illo velit')


WS.verifyElementPropertyValue(response, '[4].userId', '1')
WS.verifyElementPropertyValue(response, '[4].id', '5')
WS.verifyElementPropertyValue(response, '[4].title', 'nesciunt quas odio')
WS.verifyElementPropertyValue(response, '[4].body', 'repudiandae veniam quaerat sunt sed\nalias aut fugiat sit autem sed est\nvoluptatem omnis possimus esse voluptatibus quis\nest aut tenetur dolor neque')


WS.verifyElementPropertyValue(response, '[5].userId', '1')
WS.verifyElementPropertyValue(response, '[5].id', '6')
WS.verifyElementPropertyValue(response, '[5].title', 'dolorem eum magni eos aperiam quia')
WS.verifyElementPropertyValue(response, '[5].body', 'ut aspernatur corporis harum nihil quis provident sequi\nmollitia nobis aliquid molestiae\nperspiciatis et ea nemo ab reprehenderit accusantium quas\nvoluptate dolores velit et doloremque molestiae')


WS.verifyElementPropertyValue(response, '[6].userId', '1')
WS.verifyElementPropertyValue(response, '[6].id', '7')
WS.verifyElementPropertyValue(response, '[6].title', 'magnam facilis autem')
WS.verifyElementPropertyValue(response, '[6].body', 'dolore placeat quibusdam ea quo vitae\nmagni quis enim qui quis quo nemo aut saepe\nquidem repellat excepturi ut quia\nsunt ut sequi eos ea sed quas')


WS.verifyElementPropertyValue(response, '[7].userId', '1')
WS.verifyElementPropertyValue(response, '[7].id', '8')
WS.verifyElementPropertyValue(response, '[7].title', 'dolorem dolore est ipsam')
WS.verifyElementPropertyValue(response, '[7].body', 'dignissimos aperiam dolorem qui eum\nfacilis quibusdam animi sint suscipit qui sint possimus cum\nquaerat magni maiores excepturi\nipsam ut commodi dolor voluptatum modi aut vitae')


WS.verifyElementPropertyValue(response, '[8].userId', '1')
WS.verifyElementPropertyValue(response, '[8].id', '9')
WS.verifyElementPropertyValue(response, '[8].title', 'nesciunt iure omnis dolorem tempora et accusantium')
WS.verifyElementPropertyValue(response, '[8].body', 'consectetur animi nesciunt iure dolore\nenim quia ad\nveniam autem ut quam aut nobis\net est aut quod aut provident voluptas autem voluptas')</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
