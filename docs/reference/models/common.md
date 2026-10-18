# Common and network

<!-- Generated from the bundled SunSpec definitions by
     UPDATE_MODEL_DOCS=1 cargo test -p sunspec-parser --test models_docs
     Do not edit by hand. -->

Point names are the ones records use; `[*]` stands for a repeating group's index. See [Supported SunSpec models](README.md) for every group.

| Model | Name in records | Points |
|-------|-----------------|--------|
| [1](#model-1-common) | `common` | 6 |
| [2](#model-2-basic-aggregator) | `aggregator` | 10 |
| [3](#model-3-secure-dataset-read-request) | `model_3` | 58 |
| [4](#model-4-secure-dataset-read-response) | `model_4` | 60 |
| [5](#model-5-secure-write-request) | `model_5` | 88 |
| [6](#model-6-secure-write-sequential-request) | `model_6` | 89 |
| [7](#model-7-secure-write-response-model-draft-1) | `model_7` | 9 |
| [8](#model-8-get-device-security-certificate) | `model_8` | 3 |
| [9](#model-9-set-operator-security-certificate) | `model_9` | 92 |
| [10](#model-10-communication-interface-header) | `model_10` | 3 |
| [11](#model-11-ethernet-link-layer) | `model_11` | 7 |
| [12](#model-12-ipv4) | `model_12` | 15 |
| [13](#model-13-ipv6) | `model_13` | 15 |
| [14](#model-14-proxy-server) | `model_14` | 8 |
| [15](#model-15-interface-counters-model) | `model_15` | 12 |
| [16](#model-16-simple-ip-network) | `model_16` | 10 |
| [17](#model-17-serial-interface) | `model_17` | 8 |
| [18](#model-18-cellular-link) | `model_18` | 5 |
| [19](#model-19-ppp-link) | `model_19` | 9 |

## Model 1: Common

Name in records: `common`.

All SunSpec compliant devices must include this as the first model

| Point | Type | Units | Label | Description |
|-------|------|-------|-------|-------------|
| `Mn` | string |  | Manufacturer | Well known value registered with SunSpec for compliance |
| `Md` | string |  | Model | Manufacturer specific value (32 chars) |
| `Opt` | string |  | Options | Manufacturer specific value (16 chars) |
| `Vr` | string |  | Version | Manufacturer specific value (16 chars) |
| `SN` | string |  | Serial Number | Manufacturer specific value (32 chars) |
| `DA` | uint16 |  | Device Address | Modbus device address |

## Model 2: Basic Aggregator

Name in records: `aggregator`.

Aggregates a collection of models for a given model id

| Point | Type | Units | Label | Description |
|-------|------|-------|-------|-------------|
| `AID` | uint16 |  | AID | Aggregated model id |
| `N` | uint16 |  | N | Number of aggregated models |
| `UN` | uint16 |  | UN | Update Number. Incrementing number each time the mapping is changed. If the number is not changed from the last reading the direct access to a specific offset will result in reading the same logical model as before. Otherwise the entire model must be read to refresh the changes |
| `St` | enum16 |  | Status | Enumerated status code |
| `StVnd` | enum16 |  | Vendor Status | Vendor specific status code |
| `Evt` | bitfield32 |  | Event Code | Bitmask event code |
| `EvtVnd` | bitfield32 |  | Vendor Event Code | Vendor specific event code |
| `Ctl` | enum16 |  | Control | Control register for all aggregated devices |
| `CtlVnd` | enum32 |  | Vendor Control | Vendor control register for all aggregated devices |
| `CtlVl` | enum32 |  | Control Value | Numerical value used as a parameter to the control |

Value names:

- `St` (enum16): 1 `OFF`, 2 `ON`, 3 `FULL`, 4 `FAULT`
- `Evt` (bitfield32): bit 0 `GROUND_FAULT`, bit 1 `INPUT_OVER_VOLTAGE`, bit 2 `RESERVED_2`, bit 3 `DC_DISCONNECT`, bit 4 `RESERVED_4`, bit 5 `RESERVED_5`, bit 6 `MANUAL_SHUTDOWN`, bit 7 `OVER_TEMPERATURE`, bit 8 `RESERVED_8`, bit 9 `RESERVED_9`, bit 10 `RESERVED_10`, bit 11 `RESERVED_11`, bit 12 `BLOWN_FUSE`, bit 13 `UNDER_TEMPERATURE`, bit 14 `MEMORY_LOSS`, bit 15 `ARC_DETECTION`, bit 16 `THEFT_DETECTION`, bit 17 `OUTPUT_OVER_CURRENT`, bit 18 `OUTPUT_OVER_VOLTAGE`, bit 19 `OUTPUT_UNDER_VOLTAGE`, bit 20 `TEST_FAILED`
- `Ctl` (enum16): 0 `NONE`, 1 `AUTOMATIC`, 2 `FORCE_OFF`, 3 `TEST`, 4 `THROTTLE`

## Model 3: Secure Dataset Read Request

Name in records: `model_3`.

Request a digital signature over a specified set of data registers

| Point | Type | Units | Label | Description |
|-------|------|-------|-------|-------------|
| `X` | uint16 |  | X | Number of registers being requested |
| `Off1` | uint16 |  | Offset1 | Offset of value to read |
| `Off2` | uint16 |  |  |  |
| `Off3` | uint16 |  |  |  |
| `Off4` | uint16 |  |  |  |
| `Off5` | uint16 |  |  |  |
| `Off6` | uint16 |  |  |  |
| `Off7` | uint16 |  |  |  |
| `Off8` | uint16 |  |  |  |
| `Off9` | uint16 |  |  |  |
| `Off10` | uint16 |  |  |  |
| `Off11` | uint16 |  |  |  |
| `Off12` | uint16 |  |  |  |
| `Off13` | uint16 |  |  |  |
| `Off14` | uint16 |  |  |  |
| `Off15` | uint16 |  |  |  |
| `Off16` | uint16 |  |  |  |
| `Off17` | uint16 |  |  |  |
| `Off18` | uint16 |  |  |  |
| `Off19` | uint16 |  |  |  |
| `Off20` | uint16 |  |  |  |
| `Off21` | uint16 |  |  |  |
| `Off22` | uint16 |  |  |  |
| `Off23` | uint16 |  |  |  |
| `Off24` | uint16 |  |  |  |
| `Off25` | uint16 |  |  |  |
| `Off26` | uint16 |  |  |  |
| `Off27` | uint16 |  |  |  |
| `Off28` | uint16 |  |  |  |
| `Off29` | uint16 |  |  |  |
| `Off30` | uint16 |  |  |  |
| `Off31` | uint16 |  |  |  |
| `Off32` | uint16 |  |  |  |
| `Off33` | uint16 |  |  |  |
| `Off34` | uint16 |  |  |  |
| `Off35` | uint16 |  |  |  |
| `Off36` | uint16 |  |  |  |
| `Off37` | uint16 |  |  |  |
| `Off38` | uint16 |  |  |  |
| `Off39` | uint16 |  |  |  |
| `Off40` | uint16 |  |  |  |
| `Off41` | uint16 |  |  |  |
| `Off42` | uint16 |  |  |  |
| `Off43` | uint16 |  |  |  |
| `Off44` | uint16 |  |  |  |
| `Off45` | uint16 |  |  |  |
| `Off46` | uint16 |  |  |  |
| `Off47` | uint16 |  |  |  |
| `Off48` | uint16 |  |  |  |
| `Off49` | uint16 |  |  |  |
| `Off50` | uint16 |  |  |  |
| `Ts` | uint32 |  | Timestamp | Timestamp value is the number of seconds since January 1, 2000 |
| `Ms` | uint16 |  | Milliseconds | Millisecond counter 0-999 |
| `Seq` | uint16 |  | Sequence | Sequence number of request |
| `Role` | uint16 |  | Role | Digital Signature ID |
| `Alg` | enum16 |  | Algorithm | Algorithm used to compute the digital signature |
| `N` | uint16 |  | N | Number of registers comprising the digital signature. |
| `repeating[*].DS` | uint16 |  | DS | Digital Signature |

Value names:

- `Alg` (enum16): 0 `NONE`, 1 `AES-GMAC-64`, 2 `ECC-256`

## Model 4: Secure Dataset Read Response

Name in records: `model_4`.

Compute a digital signature over a specified set of data registers

| Point | Type | Units | Label | Description |
|-------|------|-------|-------|-------------|
| `RqSeq` | uint16 |  | Request Sequence | Sequence number from the request |
| `Sts` | enum16 |  | Status | Status of last read operation |
| `X` | uint16 |  | X | Number of values from the request |
| `Val1` | uint16 |  | Value1 | Copy of value from register Off1. |
| `Val2` | uint16 |  |  |  |
| `Val3` | uint16 |  |  |  |
| `Val4` | uint16 |  |  |  |
| `Val5` | uint16 |  |  |  |
| `Val6` | uint16 |  |  |  |
| `Val7` | uint16 |  |  |  |
| `Val8` | uint16 |  |  |  |
| `Val9` | uint16 |  |  |  |
| `Val10` | uint16 |  |  |  |
| `Val11` | uint16 |  |  |  |
| `Val12` | uint16 |  |  |  |
| `Val13` | uint16 |  |  |  |
| `Val14` | uint16 |  |  |  |
| `Val15` | uint16 |  |  |  |
| `Val16` | uint16 |  |  |  |
| `Val17` | uint16 |  |  |  |
| `Val18` | uint16 |  |  |  |
| `Val19` | uint16 |  |  |  |
| `Val20` | uint16 |  |  |  |
| `Val21` | uint16 |  |  |  |
| `Val22` | uint16 |  |  |  |
| `Val23` | uint16 |  |  |  |
| `Val24` | uint16 |  |  |  |
| `Val25` | uint16 |  |  |  |
| `Val26` | uint16 |  |  |  |
| `Val27` | uint16 |  |  |  |
| `Val28` | uint16 |  |  |  |
| `Val29` | uint16 |  |  |  |
| `Val30` | uint16 |  |  |  |
| `Val31` | uint16 |  |  |  |
| `Val32` | uint16 |  |  |  |
| `Val33` | uint16 |  |  |  |
| `Val34` | uint16 |  |  |  |
| `Val35` | uint16 |  |  |  |
| `Val36` | uint16 |  |  |  |
| `Val37` | uint16 |  |  |  |
| `Val38` | uint16 |  |  |  |
| `Val39` | uint16 |  |  |  |
| `Val40` | uint16 |  |  |  |
| `Val41` | uint16 |  |  |  |
| `Val42` | uint16 |  |  |  |
| `Val43` | uint16 |  |  |  |
| `Val44` | uint16 |  |  |  |
| `Val45` | uint16 |  |  |  |
| `Val46` | uint16 |  |  |  |
| `Val47` | uint16 |  |  |  |
| `Val48` | uint16 |  |  |  |
| `Val49` | uint16 |  |  |  |
| `Val50` | uint16 |  |  |  |
| `Ts` | uint32 |  | Timestamp | Timestamp value is the number of seconds since January 1, 2000 |
| `Ms` | uint16 |  | Milliseconds | Millisecond counter 0-999 |
| `Seq` | uint16 |  | Sequence | Sequence number of response |
| `Alm` | enum16 |  | Alarm | Bitmask alarm code |
| `Alg` | enum16 |  | Algorithm | Algorithm used to compute the digital signature |
| `N` | uint16 |  | N | Number of registers comprising the digital signature. |
| `repeating[*].DS` | uint16 |  | DS | Digital Signature |

Value names:

- `Sts` (enum16): 0 `SUCCESS`, 1 `DS`, 2 `ACL`, 3 `OFF`
- `Alm` (enum16): 0 `NONE`, 1 `ALM`
- `Alg` (enum16): 0 `NONE`, 1 `AES-GMAC-64`, 2 `ECC-256`

## Model 5: Secure Write Request

Name in records: `model_5`.

Include a digital signature along with the control data

| Point | Type | Units | Label | Description |
|-------|------|-------|-------|-------------|
| `X` | uint16 |  | X | Number of (offset, value) pairs being written |
| `Off1` | uint16 |  | Offset1 | Offset of control register to write value to |
| `Val1` | uint16 |  | Value1 | Value to write to control register at offset |
| `Off2` | uint16 |  |  |  |
| `Val2` | uint16 |  |  |  |
| `Off3` | uint16 |  |  |  |
| `Val3` | uint16 |  |  |  |
| `Off4` | uint16 |  |  |  |
| `Val4` | uint16 |  |  |  |
| `Off5` | uint16 |  |  |  |
| `Val5` | uint16 |  |  |  |
| `Off6` | uint16 |  |  |  |
| `Val6` | uint16 |  |  |  |
| `Off7` | uint16 |  |  |  |
| `Val7` | uint16 |  |  |  |
| `Off8` | uint16 |  |  |  |
| `Val8` | uint16 |  |  |  |
| `Off9` | uint16 |  |  |  |
| `Val9` | uint16 |  |  |  |
| `Off10` | uint16 |  |  |  |
| `Val10` | uint16 |  |  |  |
| `Off11` | uint16 |  |  |  |
| `Val11` | uint16 |  |  |  |
| `Off12` | uint16 |  |  |  |
| `Val12` | uint16 |  |  |  |
| `Off13` | uint16 |  |  |  |
| `Val13` | uint16 |  |  |  |
| `Off14` | uint16 |  |  |  |
| `Val14` | uint16 |  |  |  |
| `Off15` | uint16 |  |  |  |
| `Val15` | uint16 |  |  |  |
| `Off16` | uint16 |  |  |  |
| `Val16` | uint16 |  |  |  |
| `Off17` | uint16 |  |  |  |
| `Val17` | uint16 |  |  |  |
| `Off18` | uint16 |  |  |  |
| `Val18` | uint16 |  |  |  |
| `Off19` | uint16 |  |  |  |
| `Val19` | uint16 |  |  |  |
| `Off20` | uint16 |  |  |  |
| `Val20` | uint16 |  |  |  |
| `Off21` | uint16 |  |  |  |
| `Val21` | uint16 |  |  |  |
| `Off22` | uint16 |  |  |  |
| `Val22` | uint16 |  |  |  |
| `Off23` | uint16 |  |  |  |
| `Val23` | uint16 |  |  |  |
| `Off24` | uint16 |  |  |  |
| `Val24` | uint16 |  |  |  |
| `Off25` | uint16 |  |  |  |
| `Val25` | uint16 |  |  |  |
| `Off26` | uint16 |  |  |  |
| `Val26` | uint16 |  |  |  |
| `Off27` | uint16 |  |  |  |
| `Val27` | uint16 |  |  |  |
| `Off28` | uint16 |  |  |  |
| `Val28` | uint16 |  |  |  |
| `Off29` | uint16 |  |  |  |
| `Val29` | uint16 |  |  |  |
| `Off30` | uint16 |  |  |  |
| `Val30` | uint16 |  |  |  |
| `Off31` | uint16 |  |  |  |
| `Val31` | uint16 |  |  |  |
| `Off32` | uint16 |  |  |  |
| `Val32` | uint16 |  |  |  |
| `Off33` | uint16 |  |  |  |
| `Val33` | uint16 |  |  |  |
| `Off34` | uint16 |  |  |  |
| `Val34` | uint16 |  |  |  |
| `Off35` | uint16 |  |  |  |
| `Val35` | uint16 |  |  |  |
| `Off36` | uint16 |  |  |  |
| `Val36` | uint16 |  |  |  |
| `Off37` | uint16 |  |  |  |
| `Val37` | uint16 |  |  |  |
| `Off38` | uint16 |  |  |  |
| `Val38` | uint16 |  |  |  |
| `Off39` | uint16 |  |  |  |
| `Val39` | uint16 |  |  |  |
| `Off40` | uint16 |  |  |  |
| `Val40` | uint16 |  |  |  |
| `Ts` | uint32 |  | Timestamp | Timestamp value is the number of seconds since January 1, 2000 |
| `Ms` | uint16 |  | Milliseconds | Millisecond counter 0-999 |
| `Seq` | uint16 |  | Sequence | Sequence number of request |
| `Role` | uint16 |  | Role | Signing key used 0-5 |
| `Alg` | enum16 |  | Algorithm | Algorithm used to compute the digital signature |
| `N` | uint16 |  | N | Number of registers comprising the digital signature. |
| `repeating[*].DS` | uint16 |  | DS | Digital Signature |

Value names:

- `Alg` (enum16): 0 `NONE`, 1 `AES-GMAC-64`, 2 `ECC-256`

## Model 6: Secure Write Sequential Request

Name in records: `model_6`.

Include a digital signature along with the control data

| Point | Type | Units | Label | Description |
|-------|------|-------|-------|-------------|
| `X` | uint16 |  | X | Number of (offset, value) pairs being written |
| `Off` | uint16 |  | Offset | Starting offset for write operation |
| `Val1` | uint16 |  | Value1 | Value to write to control register at offset |
| `Val2` | uint16 |  |  |  |
| `Val3` | uint16 |  |  |  |
| `Val4` | uint16 |  |  |  |
| `Val5` | uint16 |  |  |  |
| `Val6` | uint16 |  |  |  |
| `Val7` | uint16 |  |  |  |
| `Val8` | uint16 |  |  |  |
| `Val9` | uint16 |  |  |  |
| `Val10` | uint16 |  |  |  |
| `Val11` | uint16 |  |  |  |
| `Val12` | uint16 |  |  |  |
| `Val13` | uint16 |  |  |  |
| `Val14` | uint16 |  |  |  |
| `Val15` | uint16 |  |  |  |
| `Val16` | uint16 |  |  |  |
| `Val17` | uint16 |  |  |  |
| `Val18` | uint16 |  |  |  |
| `Val19` | uint16 |  |  |  |
| `Val20` | uint16 |  |  |  |
| `Val21` | uint16 |  |  |  |
| `Val22` | uint16 |  |  |  |
| `Val23` | uint16 |  |  |  |
| `Val24` | uint16 |  |  |  |
| `Val25` | uint16 |  |  |  |
| `Val26` | uint16 |  |  |  |
| `Val27` | uint16 |  |  |  |
| `Val28` | uint16 |  |  |  |
| `Val29` | uint16 |  |  |  |
| `Val30` | uint16 |  |  |  |
| `Val31` | uint16 |  |  |  |
| `Val32` | uint16 |  |  |  |
| `Val33` | uint16 |  |  |  |
| `Val34` | uint16 |  |  |  |
| `Val35` | uint16 |  |  |  |
| `Val36` | uint16 |  |  |  |
| `Val37` | uint16 |  |  |  |
| `Val38` | uint16 |  |  |  |
| `Val39` | uint16 |  |  |  |
| `Val40` | uint16 |  |  |  |
| `Val41` | uint16 |  |  |  |
| `Val42` | uint16 |  |  |  |
| `Val43` | uint16 |  |  |  |
| `Val44` | uint16 |  |  |  |
| `Val45` | uint16 |  |  |  |
| `Val46` | uint16 |  |  |  |
| `Val47` | uint16 |  |  |  |
| `Val48` | uint16 |  |  |  |
| `Val49` | uint16 |  |  |  |
| `Val50` | uint16 |  |  |  |
| `Val51` | uint16 |  |  |  |
| `Val52` | uint16 |  |  |  |
| `Val53` | uint16 |  |  |  |
| `Val54` | uint16 |  |  |  |
| `Val55` | uint16 |  |  |  |
| `Val56` | uint16 |  |  |  |
| `Val57` | uint16 |  |  |  |
| `Val58` | uint16 |  |  |  |
| `Val59` | uint16 |  |  |  |
| `Val60` | uint16 |  |  |  |
| `Val61` | uint16 |  |  |  |
| `Val62` | uint16 |  |  |  |
| `Val63` | uint16 |  |  |  |
| `Val64` | uint16 |  |  |  |
| `Val65` | uint16 |  |  |  |
| `Val66` | uint16 |  |  |  |
| `Val67` | uint16 |  |  |  |
| `Val68` | uint16 |  |  |  |
| `Val69` | uint16 |  |  |  |
| `Val70` | uint16 |  |  |  |
| `Val71` | uint16 |  |  |  |
| `Val72` | uint16 |  |  |  |
| `Val73` | uint16 |  |  |  |
| `Val74` | uint16 |  |  |  |
| `Val75` | uint16 |  |  |  |
| `Val76` | uint16 |  |  |  |
| `Val77` | uint16 |  |  |  |
| `Val78` | uint16 |  |  |  |
| `Val79` | uint16 |  |  |  |
| `Val80` | uint16 |  |  |  |
| `Ts` | uint32 |  | Timestamp | Timestamp value is the number of seconds since January 1, 2000 |
| `Ms` | uint16 |  | Milliseconds | Millisecond counter 0-999 |
| `Seq` | uint16 |  | Sequence | Sequence number of request |
| `Role` | uint16 |  | Role | Signing key used 0-5 |
| `Alg` | enum16 |  | Algorithm | Algorithm used to compute the digital signature |
| `N` | uint16 |  | N | Number of registers comprising the digital signature. |
| `repeating[*].DS` | uint16 |  | DS | Digital Signature |

Value names:

- `Alg` (enum16): 0 `NONE`, 1 `AES-GMAC-64`, 2 `ECC-256`

## Model 7: Secure Write Response Model (DRAFT 1)

Name in records: `model_7`.

Include a digital signature over the response

| Point | Type | Units | Label | Description |
|-------|------|-------|-------|-------------|
| `RqSeq` | uint16 |  | Request Sequence | Sequence number from the request |
| `Sts` | enum16 |  | Status | Status of last write operation |
| `Ts` | uint32 |  | Timestamp | Timestamp value is the number of seconds since January 1, 2000 |
| `Ms` | uint16 |  | Milliseconds | Millisecond counter 0-999 |
| `Seq` | uint16 |  | Sequence | Sequence number of response |
| `Alm` | enum16 |  | Alarm | Bitmask alarm code |
| `Alg` | enum16 |  | Algorithm | Algorithm used to compute the digital signature |
| `N` | uint16 |  | N | Number of registers comprising the digital signature. |
| `repeating[*].DS` | uint16 |  | DS | Digital Signature |

Value names:

- `Sts` (enum16): 0 `SUCCESS`, 1 `DS`, 2 `ACL`, 3 `OFF`, 4 `VAL`
- `Alm` (enum16): 0 `NONE`, 1 `ALM`
- `Alg` (enum16): 0 `NONE`, 1 `AES-GMAC-64`, 2 `ECC-256`

## Model 8: Get Device Security Certificate

Name in records: `model_8`.

Security model for PKI

| Point | Type | Units | Label | Description |
|-------|------|-------|-------|-------------|
| `Fmt` | enum16 |  | Format | X.509 format of the certificate. DER or PEM. |
| `N` | uint16 |  | N | Number of registers to follow for the certificate |
| `repeating[*].Cert` | uint16 |  | Cert | X.509 Certificate of the device |

Value names:

- `Fmt` (enum16): 0 `NONE`, 1 `X509_PEM`, 2 `X509_DER`

## Model 9: Set Operator Security Certificate

Name in records: `model_9`.

Security model for PKI

| Point | Type | Units | Label | Description |
|-------|------|-------|-------|-------------|
| `CertUID` | uint16 |  | Cert\_UID | User ID for this certificate |
| `CertRole` | uint16 |  | Cert\_Role | Role for this certificate |
| `Fmt` | enum16 |  | Format | Format of this certificate |
| `Typ` | enum16 |  | Type | Type of this certificate |
| `TotLn` | uint16 |  | Total Length | Total Length of the Certificate |
| `FrgLn` | uint16 |  | Fragment length | Length of this fragment |
| `Frg1` | uint16 |  | Frag1 | First word of this fragment |
| `Frg2` | uint16 |  |  |  |
| `Frg3` | uint16 |  |  |  |
| `Frg4` | uint16 |  |  |  |
| `Frg5` | uint16 |  |  |  |
| `Frg6` | uint16 |  |  |  |
| `Frg7` | uint16 |  |  |  |
| `Frg8` | uint16 |  |  |  |
| `Frg9` | uint16 |  |  |  |
| `Frg10` | uint16 |  |  |  |
| `Frg11` | uint16 |  |  |  |
| `Frg12` | uint16 |  |  |  |
| `Frg13` | uint16 |  |  |  |
| `Frg14` | uint16 |  |  |  |
| `Frg15` | uint16 |  |  |  |
| `Frg16` | uint16 |  |  |  |
| `Frg17` | uint16 |  |  |  |
| `Frg18` | uint16 |  |  |  |
| `Frg19` | uint16 |  |  |  |
| `Frg20` | uint16 |  |  |  |
| `Frg21` | uint16 |  |  |  |
| `Frg22` | uint16 |  |  |  |
| `Frg23` | uint16 |  |  |  |
| `Frg24` | uint16 |  |  |  |
| `Frg25` | uint16 |  |  |  |
| `Frg26` | uint16 |  |  |  |
| `Frg27` | uint16 |  |  |  |
| `Frg28` | uint16 |  |  |  |
| `Frg29` | uint16 |  |  |  |
| `Frg30` | uint16 |  |  |  |
| `Frg31` | uint16 |  |  |  |
| `Frg32` | uint16 |  |  |  |
| `Frg33` | uint16 |  |  |  |
| `Frg34` | uint16 |  |  |  |
| `Frg35` | uint16 |  |  |  |
| `Frg36` | uint16 |  |  |  |
| `Frg37` | uint16 |  |  |  |
| `Frg38` | uint16 |  |  |  |
| `Frg39` | uint16 |  |  |  |
| `Frg40` | uint16 |  |  |  |
| `Frg41` | uint16 |  |  |  |
| `Frg42` | uint16 |  |  |  |
| `Frg43` | uint16 |  |  |  |
| `Frg44` | uint16 |  |  |  |
| `Frg45` | uint16 |  |  |  |
| `Frg46` | uint16 |  |  |  |
| `Frg47` | uint16 |  |  |  |
| `Frg48` | uint16 |  |  |  |
| `Frg49` | uint16 |  |  |  |
| `Frg50` | uint16 |  |  |  |
| `Frg51` | uint16 |  |  |  |
| `Frg52` | uint16 |  |  |  |
| `Frg53` | uint16 |  |  |  |
| `Frg54` | uint16 |  |  |  |
| `Frg55` | uint16 |  |  |  |
| `Frg56` | uint16 |  |  |  |
| `Frg57` | uint16 |  |  |  |
| `Frg58` | uint16 |  |  |  |
| `Frg59` | uint16 |  |  |  |
| `Frg60` | uint16 |  |  |  |
| `Frg61` | uint16 |  |  |  |
| `Frg62` | uint16 |  |  |  |
| `Frg63` | uint16 |  |  |  |
| `Frg64` | uint16 |  |  |  |
| `Frg65` | uint16 |  |  |  |
| `Frg66` | uint16 |  |  |  |
| `Frg67` | uint16 |  |  |  |
| `Frg68` | uint16 |  |  |  |
| `Frg69` | uint16 |  |  |  |
| `Frg70` | uint16 |  |  |  |
| `Frg71` | uint16 |  |  |  |
| `Frg72` | uint16 |  |  |  |
| `Frg73` | uint16 |  |  |  |
| `Frg74` | uint16 |  |  |  |
| `Frg75` | uint16 |  |  |  |
| `Frg78` | uint16 |  |  |  |
| `Frg79` | uint16 |  |  |  |
| `Frg80` | uint16 |  | Frag80 | Last word of this fragment |
| `Ts` | uint32 |  | Timestamp | Timestamp value is the number of seconds since January 1, 2000 |
| `Ms` | uint16 |  | Milliseconds | Millisecond counter 0-999 |
| `Seq` | uint16 |  | Sequence | Sequence number of request |
| `UID` | uint16 |  | UID | User ID for the request signature |
| `Role` | uint16 |  | Role | Signing key used 0-5 |
| `Alg` | enum16 |  | Algorithm | Algorithm used to compute the digital signature |
| `N` | uint16 |  | N | Number of registers to follow for the certificate |
| `repeating[*].Cert` | uint16 |  |  |  |

Value names:

- `Fmt` (enum16): 0 `NONE`, 1 `X509_PEM`, 2 `X509_DER`
- `Typ` (enum16): 0 `DEV_KEY_PAIR`, 1 `DEV_SHARED_KEY`, 2 `OPERATOR_PUB`, 3 `OPERATOR_SHARED`, 4 `CA_PUB`
- `Alg` (enum16): 0 `NONE`, 1 `AES-GMAC-64`, 2 `ECC-256`

## Model 10: Communication Interface Header

Name in records: `model_10`.

To be included first for a complete interface description

| Point | Type | Units | Label | Description |
|-------|------|-------|-------|-------------|
| `St` | enum16 |  | Interface Status | Overall interface status |
| `Ctl` | uint16 |  | Interface Control | Overall interface control (TBD) |
| `Typ` | enum16 |  | Physical Access Type | Type of physical media |

Value names:

- `St` (enum16): 0 `DOWN`, 1 `UP`, 2 `FAULT`
- `Typ` (enum16): 0 `UNKNOWN`, 1 `INTERNAL`, 2 `TWISTED_PAIR`, 3 `FIBER`, 4 `WIRELESS`

## Model 11: Ethernet Link Layer

Name in records: `model_11`.

Include to support a wired ethernet port

| Point | Type | Units | Label | Description |
|-------|------|-------|-------|-------------|
| `Spd` | uint16 | Mbps | Ethernet Link Speed | Interface speed in Mb/s |
| `CfgSt` | bitfield16 |  | Interface Status Flags | Interface flags. |
| `St` | enum16 |  | Link State | State information for this interface |
| `MAC` | eui48 |  | MAC | IEEE MAC address of this interface |
| `Nam` | string |  | Name | Interface name (8 chars) |
| `Ctl` | bitfield16 |  | Control | Control flags |
| `FrcSpd` | uint16 | Mbps | Forced Speed | Forced interface speed in Mb/s when AUTO is disabled |

Value names:

- `CfgSt` (bitfield16): bit 0 `LINK`, bit 1 `FULL_DUPLEX`, bit 2 `AUTO_NEG1`, bit 3 `AUTO_NEG2`, bit 4 `AUTO_NEG3`, bit 5 `RESET_REQUIRED`, bit 6 `HW_FAULT`
- `St` (enum16): 0 `UNKNOWN`, 1 `ENABLED`, 2 `DISABLED`, 3 `TESTING`
- `Ctl` (bitfield16): bit 0 `AUTO`, bit 1 `FULL_DUPLEX`

## Model 12: IPv4

Name in records: `model_12`.

Include to support an IPv4 protocol stack on this interface

| Point | Type | Units | Label | Description |
|-------|------|-------|-------|-------------|
| `Nam` | string |  | Name | Interface name |
| `CfgSt` | enum16 |  | Config Status | Configuration status |
| `ChgSt` | bitfield16 |  | Change Status | A configuration change is pending |
| `Cap` | bitfield16 |  | Config Capability | Identify capable sources of configuration |
| `Cfg` | enum16 |  | IPv4 Config | Configuration method used. |
| `Ctl` | enum16 |  | Control | Configure use of services |
| `Addr` | string |  | IP | IPv4 numeric address as a dotted string xxx.xxx.xxx.xxx |
| `Msk` | string |  | Netmask | IPv4 numeric netmask as a dotted string xxx.xxx.xxx.xxx |
| `Gw` | string |  | Gateway | IPv4 numeric gateway address as a dotted string xxx.xxx.xxx.xxx |
| `DNS1` | string |  | DNS1 | IPv4 numeric DNS address as a dotted string xxx.xxx.xxx.xxx |
| `DNS2` | string |  | DNS2 | IPv4 numeric DNS address as a dotted string xxx.xxx.xxx.xxx |
| `NTP1` | string |  | NTP1 | IPv4 numeric NTP address as a dotted string xxx.xxx.xxx.xxx |
| `NTP2` | string |  | NTP2 | IPv4 numeric NTP address as a dotted string xxx.xxx.xxx.xxx |
| `DomNam` | string |  | Domain | Domain name (24 chars max) |
| `HostNam` | string |  | Host Name | Host name (24 chars max) |

Value names:

- `CfgSt` (enum16): 0 `NOT_CONFIGURED`, 1 `VALID_SETTING`, 2 `VALID_HW`
- `ChgSt` (bitfield16): bit 0 `PENDING`
- `Cap` (bitfield16): bit 0 `DHCP`, bit 1 `BOOTP`, bit 2 `ZEROCONF`, bit 3 `DNS`, bit 4 `CFG_SETTABLE`, bit 5 `HW_CONFIG`, bit 6 `NTP_CLIENT`, bit 7 `RESET_REQUIRED`
- `Cfg` (enum16): 0 `STATIC`, 1 `DHCP`, 2 `BOOTP`, 3 `ZEROCONF`
- `Ctl` (enum16): 0 `ENABLE_DNS`, 1 `ENABLE_NTP`

## Model 13: IPv6

Name in records: `model_13`.

Include to support an IPv6 protocol stack on this interface

| Point | Type | Units | Label | Description |
|-------|------|-------|-------|-------------|
| `Nam` | string |  | Name | Interface name |
| `CfgSt` | enum16 |  | Config Status | Configuration status |
| `ChgSt` | bitfield16 |  | Change Status | A configuration change is pending |
| `Cap` | bitfield16 |  | Config Capability | Identify capable sources of configuration |
| `Cfg` | enum16 |  | IPv6 Config | Configuration method used. |
| `Ctl` | enum16 |  | Control | Configure use of services |
| `Addr` | string |  | IP | IPv6 numeric address as a dotted string xxxx.xxxx.xxxx.xxxx |
| `CIDR` | string |  | CIDR | Classless Inter-Domain Routing Number |
| `Gw` | string |  | Gateway | IPv6 numeric address as a dotted string xxxx.xxxx.xxxx.xxxx |
| `DNS1` | string |  | DNS1 | IPv6 numeric DNS address as a dotted string xxxx.xxxx.xxxx.xxxx |
| `DNS2` | string |  | DNS2 | IPv6 numeric DNS address as a dotted string xxxx.xxxx.xxxx.xxxx |
| `NTP1` | string |  | NTP1 | IPv6 numeric NTP address as a name or dotted string xxxx.xxxx.xxxx.xxxx |
| `NTP2` | string |  | NTP2 | IPv6 numeric NTP address as a name or dotted string xxxx.xxxx.xxxx.xxxx |
| `DomNam` | string |  | Domain | Domain name (24 chars max) |
| `HostNam` | string |  | Host Name | Host name (24 chars max) |

Value names:

- `CfgSt` (enum16): 0 `NOT_CONFIGURED`, 1 `VALID_SETTING`, 2 `VALID_HW`
- `ChgSt` (bitfield16): bit 0 `PENDING`
- `Cap` (bitfield16): bit 0 `DHCP`, bit 1 `BOOTP`, bit 2 `ZEROCONF`, bit 3 `DNS`, bit 4 `CFG_SETTABLE`, bit 5 `HW_CONFIG`, bit 6 `NTP_CLIENT`, bit 7 `RESET_REQUIRED`
- `Cfg` (enum16): 0 `STATIC`, 1 `DHCP`, 2 `BOOTP`, 3 `ZEROCONF`
- `Ctl` (enum16): 0 `ENABLE_DNS`, 1 `ENABLE_NTP`

## Model 14: Proxy Server

Name in records: `model_14`.

Include this block to allow for a proxy server

| Point | Type | Units | Label | Description |
|-------|------|-------|-------|-------------|
| `Nam` | string |  | name | Interface name (8 chars) |
| `Cap` | bitfield16 |  | Capabilities | Proxy configuration capabilities |
| `Cfg` | enum16 |  | Config | Set proxy address type |
| `Typ` | bitfield16 |  | Type | Enumerate value. Proxy server type |
| `Addr` | string |  | Address | IPv4 or IPv6 proxy hostname or dotted address (40 chars) |
| `Port` | uint16 |  | Port | Proxy port number |
| `User` | string |  | Username | Proxy user name |
| `Pw` | string |  | Password | Proxy password |

Value names:

- `Cap` (bitfield16): bit 0 `NO_PROXY`, bit 1 `IPV4_PROXY`, bit 2 `IPV6_PROXY`

## Model 15: Interface Counters Model

Name in records: `model_15`.

Interface counters

| Point | Type | Units | Label | Description |
|-------|------|-------|-------|-------------|
| `Clr` | uint16 |  | Clear | Write a "1" to clear all counters |
| `InCnt` | acc32 |  | Input Count | Number of bytes received |
| `InUcCnt` | acc32 |  | Input Unicast Count | Number of Unicast packets received |
| `InNUcCnt` | acc32 |  | Input Non-Unicast Count | Number of non-Unicast packets received |
| `InDscCnt` | acc32 |  | Input Discarded Count | Number of inbound packets received on the interface but discarded |
| `InErrCnt` | acc32 |  | Input Error Count | Number of inbound packets that contain errors (excluding discards) |
| `InUnkCnt` | acc32 |  | Input Unknown Count | Number of inbound packets with unknown protocol |
| `OutCnt` | acc32 |  | Output Count | Total number of bytes transmitted on this interface |
| `OutUcCnt` | acc32 |  | Output Unicast Count | Number of Unicast packets transmitted |
| `OutNUcCnt` | acc32 |  | Output Non-Unicast Count | Number of Non-Unicast packets transmitted |
| `OutDscCnt` | acc32 |  | Output Discarded Count | Number of Discarded output packets |
| `OutErrCnt` | acc32 |  | Output Error Count | Number of outbound error packets |

## Model 16: Simple IP Network

Name in records: `model_16`.

Include this model for a simple IPv4 network stack

| Point | Type | Units | Label | Description |
|-------|------|-------|-------|-------------|
| `Nam` | string |  | Name | Interface name. (8 chars) |
| `Cfg` | enum16 |  | Config | Force IPv4 configuration method |
| `Ctl` | bitfield16 |  | Control | Configure use of services |
| `Addr` | string |  | Address | IP address |
| `Msk` | string |  | Netmask | Netmask |
| `Gw` | string |  | Gateway | Gateway IP address |
| `DNS1` | string |  | DNS1 | 32 bit IP address of DNS server |
| `DNS2` | string |  | DNS2 | 32 bit IP address of DNS server |
| `MAC` | eui48 |  | MAC | IEEE MAC address of this interface |
| `LnkCtl` | bitfield16 |  | Link Control | Link control flags |

Value names:

- `Cfg` (enum16): 0 `STATIC`, 1 `DHCP`
- `Ctl` (bitfield16): bit 0 `ENABLE_DNS`, bit 1 `ENABLE_NTP`
- `LnkCtl` (bitfield16): bit 0 `AUTONEGOTIATE`, bit 1 `FULL_DUPLEX`, bit 2 `FORCE_10MB`, bit 3 `FORCE_100MB`, bit 4 `FORCE_1GB`

## Model 17: Serial Interface

Name in records: `model_17`.

Include this model for serial interface configuration support

| Point | Type | Units | Label | Description |
|-------|------|-------|-------|-------------|
| `Nam` | string |  | Name | Interface name (8 chars) |
| `Rte` | uint32 | bps | Rate | Interface baud rate in bits per second |
| `Bits` | uint16 |  | Bits | Number of data bits per character |
| `Pty` | enum16 |  | Parity | Parity setting |
| `Dup` | enum16 |  | Duplex | Duplex mode |
| `Flw` | enum16 |  | Flow Control | Flow Control Method |
| `Typ` | enum16 |  | Interface Type | Interface type |
| `Pcol` | enum16 |  | Protocol | Serial protocol selection |

Value names:

- `Pty` (enum16): 0 `NONE`, 1 `ODD`, 2 `EVEN`
- `Dup` (enum16): 0 `FULL`, 1 `HALF`
- `Flw` (enum16): 0 `NONE`, 1 `HW`, 2 `XONXOFF`
- `Typ` (enum16): 0 `UNKNOWN`, 1 `RS232`, 2 `RS485`
- `Pcol` (enum16): 0 `UNKNOWN`, 1 `MODBUS`, 2 `VENDOR`

## Model 18: Cellular Link

Name in records: `model_18`.

Include this model to support a cellular interface link

| Point | Type | Units | Label | Description |
|-------|------|-------|-------|-------------|
| `Nam` | string |  | Name | Interface name |
| `IMEI` | uint32 |  | IMEI | International Mobile Equipment Identifier for the interface |
| `APN` | string |  | APN | Access Point Name for the interface |
| `Num` | string |  | Number | Phone number for the interface |
| `Pin` | string |  | PIN | Personal Identification Number for the interface |

## Model 19: PPP Link

Name in records: `model_19`.

Include this model to configure a Point-to-Point Protocol link

| Point | Type | Units | Label | Description |
|-------|------|-------|-------|-------------|
| `Nam` | string |  | Name | Interface name |
| `Rte` | uint32 | bps | Rate | Interface baud rate in bits per second |
| `Bits` | uint16 |  | Bits | Number of data bits per character |
| `Pty` | enum16 |  | Parity | Parity setting |
| `Dup` | enum16 |  | Duplex | Duplex mode |
| `Flw` | enum16 |  | Flow Control | Flow Control Method |
| `Auth` | enum16 |  | Authentication | Authentication method |
| `UsrNam` | string |  | Username | Username for authentication |
| `Pw` | string |  | Password | Password for authentication |

Value names:

- `Pty` (enum16): 0 `NONE`, 1 `ODD`, 2 `EVEN`
- `Dup` (enum16): 0 `FULL`, 1 `HALF`
- `Flw` (enum16): 0 `NONE`, 1 `HW`, 2 `XONXOFF`
- `Auth` (enum16): 0 `NONE`, 1 `PAP`, 2 `CHAP`
