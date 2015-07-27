// Copyright © 2015, skdltmxn
// Licensed under the MIT License <LICENSE.md>
#![cfg(windows)]
extern crate crypt32;
use crypt32::*;
#[inline(never)] fn bb<T>(_: T) {}
#[test]
fn functions() {
    bb(CertAddCRLContextToStore);
    bb(CertAddCRLLinkToStore);
    bb(CertAddCTLContextToStore);
    bb(CertAddCTLLinkToStore);
    bb(CertAddCertificateContextToStore);
    bb(CertAddCertificateLinkToStore);
    bb(CertAddEncodedCRLToStore);
    bb(CertAddEncodedCTLToStore);
    bb(CertAddEncodedCertificateToStore);
    bb(CertAddEncodedCertificateToSystemStoreA);
    bb(CertAddEncodedCertificateToSystemStoreW);
    bb(CertAddEnhancedKeyUsageIdentifier);
    bb(CertAddSerializedElementToStore);
    bb(CertAddStoreToCollection);
    bb(CertAlgIdToOID);
    bb(CertCloseStore);
    bb(CertCompareCertificate);
    bb(CertCompareCertificateName);
    bb(CertCompareIntegerBlob);
    bb(CertComparePublicKeyInfo);
    bb(CertControlStore);
    bb(CertCreateCRLContext);
    bb(CertCreateCTLContext);
    bb(CertCreateCTLEntryFromCertificateContextProperties);
    bb(CertCreateCertificateChainEngine);
    bb(CertCreateCertificateContext);
    bb(CertCreateContext);
    bb(CertCreateSelfSignCertificate);
    bb(CertDeleteCRLFromStore);
    bb(CertDeleteCTLFromStore);
    bb(CertDeleteCertificateFromStore);
    bb(CertDuplicateCRLContext);
    bb(CertDuplicateCTLContext);
    bb(CertDuplicateCertificateChain);
    bb(CertDuplicateCertificateContext);
    bb(CertDuplicateStore);
    bb(CertEnumCRLContextProperties);
    bb(CertEnumCRLsInStore);
    bb(CertEnumCTLContextProperties);
    bb(CertEnumCTLsInStore);
    bb(CertEnumCertificateContextProperties);
    bb(CertEnumCertificatesInStore);
    bb(CertEnumPhysicalStore);
    bb(CertEnumSubjectInSortedCTL);
    bb(CertEnumSystemStore);
    bb(CertEnumSystemStoreLocation);
    bb(CertFindAttribute);
    bb(CertFindCRLInStore);
    bb(CertFindCTLInStore);
    bb(CertFindCertificateInCRL);
    bb(CertFindCertificateInStore);
    bb(CertFindChainInStore);
    bb(CertFindExtension);
    bb(CertFindRDNAttr);
    bb(CertFindSubjectInCTL);
    bb(CertFindSubjectInSortedCTL);
    bb(CertFreeCRLContext);
    bb(CertFreeCTLContext);
    bb(CertFreeCertificateChain);
    bb(CertFreeCertificateChainEngine);
    bb(CertFreeCertificateContext);
    bb(CertGetCRLContextProperty);
    bb(CertGetCRLFromStore);
    bb(CertGetCTLContextProperty);
    bb(CertGetCertificateChain);
    bb(CertGetCertificateContextProperty);
    bb(CertGetEnhancedKeyUsage);
    bb(CertGetIntendedKeyUsage);
    bb(CertGetIssuerCertificateFromStore);
    bb(CertGetNameStringA);
    bb(CertGetNameStringW);
    bb(CertGetPublicKeyLength);
    bb(CertGetStoreProperty);
    bb(CertGetSubjectCertificateFromStore);
    bb(CertGetValidUsages);
    bb(CertIsRDNAttrsInCertificateName);
    bb(CertIsValidCRLForCertificate);
    bb(CertNameToStrA);
    bb(CertNameToStrW);
    bb(CertOIDToAlgId);
    bb(CertOpenStore);
    bb(CertOpenSystemStoreA);
    bb(CertOpenSystemStoreW);
    bb(CertRDNValueToStrA);
    bb(CertRDNValueToStrW);
    bb(CertRegisterPhysicalStore);
    bb(CertRegisterSystemStore);
    bb(CertRemoveEnhancedKeyUsageIdentifier);
    bb(CertRemoveStoreFromCollection);
    bb(CertResyncCertificateChainEngine);
    bb(CertSaveStore);
    bb(CertSerializeCRLStoreElement);
    bb(CertSerializeCTLStoreElement);
    bb(CertSerializeCertificateStoreElement);
    bb(CertSetCRLContextProperty);
    bb(CertSetCTLContextProperty);
    bb(CertSetCertificateContextPropertiesFromCTLEntry);
    bb(CertSetCertificateContextProperty);
    bb(CertSetEnhancedKeyUsage);
    bb(CertSetStoreProperty);
    bb(CertStrToNameA);
    bb(CertStrToNameW);
    bb(CertUnregisterPhysicalStore);
    bb(CertUnregisterSystemStore);
    bb(CertVerifyCRLRevocation);
    bb(CertVerifyCRLTimeValidity);
    bb(CertVerifyCTLUsage);
    bb(CertVerifyCertificateChainPolicy);
    bb(CertVerifyRevocation);
    bb(CertVerifySubjectCertificateContext);
    bb(CertVerifyTimeValidity);
    bb(CertVerifyValidityNesting);
    bb(CryptAcquireCertificatePrivateKey);
    bb(CryptBinaryToStringA);
    bb(CryptBinaryToStringW);
    bb(CryptCloseAsyncHandle);
    bb(CryptCreateAsyncHandle);
    bb(CryptCreateKeyIdentifierFromCSP);
    bb(CryptDecodeMessage);
    bb(CryptDecodeObject);
    bb(CryptDecodeObjectEx);
    bb(CryptDecryptAndVerifyMessageSignature);
    bb(CryptDecryptMessage);
    bb(CryptEncodeObject);
    bb(CryptEncodeObjectEx);
    bb(CryptEncryptMessage);
    bb(CryptEnumKeyIdentifierProperties);
    bb(CryptEnumOIDFunction);
    bb(CryptEnumOIDInfo);
    bb(CryptExportPKCS8);
    bb(CryptExportPublicKeyInfo);
    bb(CryptExportPublicKeyInfoEx);
    bb(CryptFindCertificateKeyProvInfo);
    bb(CryptFindLocalizedName);
    bb(CryptFindOIDInfo);
    bb(CryptFormatObject);
    bb(CryptFreeOIDFunctionAddress);
    bb(CryptGetAsyncParam);
    bb(CryptGetDefaultOIDDllList);
    bb(CryptGetDefaultOIDFunctionAddress);
    bb(CryptGetKeyIdentifierProperty);
    bb(CryptGetMessageCertificates);
    bb(CryptGetMessageSignerCount);
    bb(CryptGetOIDFunctionAddress);
    bb(CryptGetOIDFunctionValue);
    bb(CryptHashCertificate);
    bb(CryptHashMessage);
    bb(CryptHashPublicKeyInfo);
    bb(CryptHashToBeSigned);
    bb(CryptImportPKCS8);
    bb(CryptImportPublicKeyInfo);
    bb(CryptImportPublicKeyInfoEx);
    bb(CryptInitOIDFunctionSet);
    bb(CryptInstallDefaultContext);
    bb(CryptInstallOIDFunctionAddress);
    bb(CryptMemAlloc);
    bb(CryptMemFree);
    bb(CryptMemRealloc);
    bb(CryptMsgCalculateEncodedLength);
    bb(CryptMsgClose);
    bb(CryptMsgControl);
    bb(CryptMsgCountersign);
    bb(CryptMsgCountersignEncoded);
    bb(CryptMsgDuplicate);
    bb(CryptMsgEncodeAndSignCTL);
    bb(CryptMsgGetAndVerifySigner);
    bb(CryptMsgGetParam);
    bb(CryptMsgOpenToDecode);
    bb(CryptMsgOpenToEncode);
    bb(CryptMsgSignCTL);
    bb(CryptMsgUpdate);
    bb(CryptMsgVerifyCountersignatureEncoded);
    bb(CryptMsgVerifyCountersignatureEncodedEx);
    bb(CryptProtectData);
    bb(CryptQueryObject);
    bb(CryptRegisterDefaultOIDFunction);
    bb(CryptRegisterOIDFunction);
    bb(CryptRegisterOIDInfo);
    bb(CryptSIPAddProvider);
    bb(CryptSIPCreateIndirectData);
    bb(CryptSIPGetSignedDataMsg);
    bb(CryptSIPLoad);
    bb(CryptSIPPutSignedDataMsg);
    bb(CryptSIPRemoveProvider);
    bb(CryptSIPRemoveSignedDataMsg);
    bb(CryptSIPRetrieveSubjectGuid);
    bb(CryptSIPVerifyIndirectData);
    bb(CryptSetAsyncParam);
    bb(CryptSetKeyIdentifierProperty);
    bb(CryptSetOIDFunctionValue);
    bb(CryptSignAndEncodeCertificate);
    bb(CryptSignAndEncryptMessage);
    bb(CryptSignCertificate);
    bb(CryptSignMessage);
    bb(CryptSignMessageWithKey);
    bb(CryptStringToBinaryA);
    bb(CryptStringToBinaryW);
    bb(CryptUninstallDefaultContext);
    bb(CryptUnprotectData);
    bb(CryptUnregisterDefaultOIDFunction);
    bb(CryptUnregisterOIDFunction);
    bb(CryptUnregisterOIDInfo);
    bb(CryptVerifyCertificateSignature);
    bb(CryptVerifyCertificateSignatureEx);
    bb(CryptVerifyDetachedMessageHash);
    bb(CryptVerifyDetachedMessageSignature);
    bb(CryptVerifyMessageHash);
    bb(CryptVerifyMessageSignature);
    bb(CryptVerifyMessageSignatureWithKey);
    bb(PFXExportCertStore);
    bb(PFXExportCertStoreEx);
    bb(PFXImportCertStore);
    bb(PFXIsPFXBlob);
    bb(PFXVerifyPassword);
}
#[cfg(target_env = "msvc")]
#[test]
fn functions_msvc() {
    bb(CertAddRefServerOcspResponse);
    bb(CertAddRefServerOcspResponseContext);
    bb(CertCloseServerOcspResponse);
    bb(CertFreeServerOcspResponseContext);
    bb(CertGetServerOcspResponseContext);
    bb(CertFreeCertificateChainList);
    bb(CertOpenServerOcspResponse);
    bb(CertIsStrongHashToSign);
    bb(CertRetrieveLogoOrBiometricInfo);
    bb(CertSelectCertificateChains);
    bb(CryptExportPublicKeyInfoFromBCryptKeyHandle);
    bb(CryptHashCertificate2);
    bb(CryptImportPublicKeyInfoEx2);
    bb(CryptRetrieveTimeStamp);
    bb(CryptSIPGetCaps);
    bb(CryptSIPGetSealedDigest);
    bb(CryptUpdateProtectedState);
    bb(CryptVerifyTimeStampSignature);
    bb(CryptProtectMemory);
    bb(CryptUnprotectMemory);
}