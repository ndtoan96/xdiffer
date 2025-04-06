export const demoXml1 = `<?xml version="1.0"?>
<Profile>
    <Customer>
        <PersonName NameType="Default">
            <NameTitle>Mr.</NameTitle>
            <GivenName>George</GivenName>
            <MiddleName>A.</MiddleName>
            <SurName>Smith</SurName>
            <Bio>A skilled engineer with a passion for solving complex problems through innovative
                design and efficient solutions. With expertise in software development, mechanical
                engineering, he has worked on projects ranging from nuclear reactor to spaceship.
                Always eager to learn and adapt, he thrives in collaborative environments,
                leveraging technical knowledge to drive impactful results.</Bio>
        </PersonName>
        <TelephoneInfo PhoneTech="Voice" PhoneUse="Work">
            <Telephone>
                <AreaCityCode>206</AreaCityCode>
                <PhoneNumber>813-8698</PhoneNumber>
            </Telephone>
        </TelephoneInfo>
        <PaymentForm>
            ...
        </PaymentForm>
        <Address>
            <StreetNmbr POBox="4321-01">From hell</StreetNmbr>
            <BldgRoom>Suite 800</BldgRoom>
            <CityName>Seattle</CityName>
            <StateProv PostalCode="98108">WA</StateProv>
            <CountryName>USA</CountryName>
        </Address>
        <Address>
            <StreetNmbr POBox="4321-01">1200 Yakima St</StreetNmbr>
            <BldgRoom>Suite 800</BldgRoom>
            <CityName>Seattle</CityName>
            <StateProv PostalCode="98108">WA</StateProv>
            <CountryName>USA</CountryName>
        </Address>
    </Customer>
</Profile>`;

export const demoXml2 = `<?xml version="1.0"?>
<Profile>
 <Customer>
  <PersonName NameType="Default" Foo="foobar">
   <NameTitle>Mr.</NameTitle>
   <GivenName Attr="cool">
	Fred
   </GivenName>
   <MiddleName>A.</MiddleName>
   <SurName>Smith</SurName>
   <NewStuff>Smurf</NewStuff>
  </PersonName>
  <TelephoneInfo PhoneTech="Voice" PhoneUse="Work" >
   <Telephone>       <AreaCityCode>206</AreaCityCode>
	<PhoneNumber>
   813-8698
   </PhoneNumber>
   </Telephone>
  </TelephoneInfo>
  <PaymentForm>
   ...
  </PaymentForm>
  <Address>
   <StreetNmbr POBox="4321-01">1200 Yakima St</StreetNmbr>
   <BldgRoom>Suite 800</BldgRoom>
   <CityName>Seattle</CityName>
   <StateProv PostalCode="98108">WA</StateProv>
   <CountryName>USA</CountryName>
   <Planet>Earth</Planet>
  </Address>
  <RelatedTraveler Relation="Child">
   <PersonName>
	<GivenName>Devin</GivenName>
	<MiddleName>R.</MiddleName>
	<SurName>Smith</SurName>
   </PersonName>
  </RelatedTraveler>
  <RelatedTraveler Relation="Child">
   <PersonName>
	<GivenName>Amy</GivenName>
	<MiddleName>E.</MiddleName>
	<SurName>Smith</SurName>
   </PersonName>
  </RelatedTraveler>
  <RelatedTraveler Relation="Child">
   <PersonName>
	<GivenName>Alfred</GivenName>
	<MiddleName>E.</MiddleName>
	<SurName>Newman</SurName>
   </PersonName>
  </RelatedTraveler>
 </Customer>
</Profile>`;