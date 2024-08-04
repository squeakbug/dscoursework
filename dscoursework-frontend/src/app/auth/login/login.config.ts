import { AuthConfig } from 'angular-oauth2-oidc';

export const authCodeFlowConfig: AuthConfig = {
    issuer: 'https://localhost:9443/application/o/dscoursework/',
    redirectUri: 'https://localhost:4200/login',
    clientId: 'W6awIjORjnwvxbfLtIwUpSRWURe105TNqsRy2oDi',
    dummyClientSecret: 'Cl38WPWRM42iUSsjoQ6XyIX1F1n49a8wv1h6DGkZ58TJAd88FJFkfRfB8ARpcHacAkXlQaWB29jCXZFylD09FDrHlIvOzizlRiZCViaBujDdGXjGss1hUydPl8SXzupc',
    responseType: 'code',
    scope: 'openid profile email offline_access api',
    showDebugInformation: true,
    strictDiscoveryDocumentValidation: false,
};
