
/*
Initiate a transaction
When the customer is ready to make payment the merchant site must perform an Initiate Transaction request. This request is in the form of an HTTP POST to the URL:

https://www.paynow.co.zw/interface/initiatetransaction

The HTTP POST should include the following fields:

Field	Data Type	Description
id	Integer	Integration ID shown to the merchant in the “3rd Party Site or Link Profile” area of “Receive Payment Links” section of “Sell or Receive” on Paynow.
reference	String	The transaction’s reference on the merchant site, this should be unique to the transaction.
amount	Decimal	Final amount of the transaction to two decimal places (do not include a currency symbol).
additionalinfo	String	(optional) Additional info to be displayed on Paynow to the Customer. This should not include any confidential information.
returnurl	String	The URL on the merchant website the customer will be returned to after the transaction has been processed. It is recommended this URL contains enough information for the merchant site to identify the transaction.
resulturl	String	The URL on the merchant website Paynow will post transaction results to. It is recommended this URL contains enough information for the merchant site to identify the transaction.
authemail	String	(optional) If the field is present and set to an email address Paynow will attempt to auto login the customers email address as an anonymous user. If the email address has a registered account the user will be prompted to login to that account.

N.B. This field is required when initiating Express Checkout Transactions
tokenize	Boolean	(optional) If set to true and the customer uses Visa/Mastercard to complete the transaction, the tokenized payment instrument will be returned in the status update which can be used for recurring payments without requiring further input from the customer.

N.B. A merchant may only use this field if permitted to tokenize payment instruments. Contact [email protected] to apply for this functionality.
status	String	Should be set to “Message” at this stage of the transaction.
hash	String	Details of Hash generation are provided in the Generating Hash section.
*/

//TODO refactor transaction formatting using traits



struct InitTxn ; // 

struct InitExpressCheckoutTxn;

struct InitPassengerTicketTxn;

// Write init transaction POST
/*
Field	Data Type	Description
id	Integer	Integration ID shown to the merchant in the “3rd Party Site or Link Profile” area of “Receive Payment Links” section of “Sell or Receive” on Paynow.
reference	String	The transaction’s reference on the merchant site, this should be unique to the transaction.
amount	Decimal	Final amount of the transaction to two decimal places (do not include a currency symbol).
additionalinfo	String	(optional) Additional info to be displayed on Paynow to the Customer. This should not include any confidential information.
returnurl	String	The URL on the merchant website the customer will be returned to after the transaction has been processed. It is recommended this URL contains enough information for the merchant site to identify the transaction.
resulturl	String	The URL on the merchant website Paynow will post transaction results to. It is recommended this URL contains enough information for the merchant site to identify the transaction.
authemail	String	(optional) If the field is present and set to an email address Paynow will attempt to auto login the customers email address as an anonymous user. If the email address has a registered account the user will be prompted to login to that account.

N.B. This field is required when initiating Express Checkout Transactions
tokenize	Boolean	(optional) If set to true and the customer uses Visa/Mastercard to complete the transaction, the tokenized payment instrument will be returned in the status update which can be used for recurring payments without requiring further input from the customer.

N.B. A merchant may only use this field if permitted to tokenize payment instruments. Contact support@paynow.co.zw to apply for this functionality.
status	String	Should be set to “Message” at this stage of the transaction.
hash	String
*/


