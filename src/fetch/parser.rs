//! Parses an HTML body to build an [`Email`] object

use std::collections::HashMap;

use mail_parser::{HeaderName, HeaderValue, MessageParser};

use crate::errors::Result;

/// Headers of an email
type Headers = HashMap<HeaderName<'static>, HeaderValue<'static>>;

//TODO: this doesn't support nested messages yet. See mail-parser attachments
// to this extent.
/// Represents a parsed email
pub struct Email {
    /// Headers of the email
    ///
    /// This contains the date, the origin (`from`), the destination (`to`,
    /// `cc`, `bcc`), the subject, etc.
    headers: Headers,
    /// HTML version of the email content
    html: Option<String>,
    /// Plain text version of the email content
    text: Option<String>,
    /// Unique ID corresponding to the email
    uid: u32,
}

impl Email {
    /// Returns the headers of the email
    pub const fn as_headers(&self) -> &Headers {
        &self.headers
    }

    /// Returns the value of a header
    pub fn get_header(&self, header_name: &HeaderName<'_>) -> Result<HeaderValue<'_>> {
        Ok(self
            .as_headers()
            .get(header_name)
            .ok_or(Error::MissingHeader)?
            .to_owned())
    }
}

impl<'body> TryFrom<(u32, &'body [u8])> for Email {
    type Error = Error;

    fn try_from((uid, value): (u32, &'body [u8])) -> Result<Self, Error> {
        let message = MessageParser::default()
            .parse(value)
            .ok_or(Error::ParseFailure)?;

        let headers = message
            .parts
            .first()
            .ok_or(Error::NoHeaders)?
            .headers
            .iter()
            .map(|header| (header.name.to_owned(), header.value.clone().into_owned()))
            .collect();

        let html = message.body_html(0).map(|html| html.to_string());
        let text = message.body_text(0).map(|text| text.to_string());

        Ok(Self { headers, html, text, uid })
    }
}

/// Errors that may occur while parsing the email.
#[derive(Debug)]
pub enum Error {
    /// Given header has the wrong type
    InvalidHeaderType,
    /// Failed to parse the email.
    ParseFailure,
    /// Failed to get headers from the email.
    NoHeaders,
    /// Failed to get the wanted header
    MissingHeader,
}

#[cfg(test)]
mod test {
    use mail_parser::{Addr, Group, HeaderName};

    use crate::fetch::parser::Email;

    const EMAIL_EXAMPLES: &[u8] = br#"From: Art Vandelay <art@vandelay.com> (Vandelay Industries)
To: "Colleagues": "James Smythe" <james@vandelay.com>; Friends:
    jane@example.com, =?UTF-8?Q?John_Sm=C3=AEth?= <john@example.com>;
Date: Sat, 20 Nov 2021 14:22:01 -0800
Subject: Why not both importing AND exporting? =?utf-8?b?4pi6?=
Content-Type: multipart/mixed; boundary="festivus";

--festivus
Content-Type: text/html; charset="us-ascii"
Content-Transfer-Encoding: base64

PGh0bWw+PHA+SSB3YXMgdGhpbmtpbmcgYWJvdXQgcXVpdHRpbmcgdGhlICZsZHF1bztle
HBvcnRpbmcmcmRxdW87IHRvIGZvY3VzIGp1c3Qgb24gdGhlICZsZHF1bztpbXBvcnRpbm
cmcmRxdW87LDwvcD48cD5idXQgdGhlbiBJIHRob3VnaHQsIHdoeSBub3QgZG8gYm90aD8
gJiN4MjYzQTs8L3A+PC9odG1sPg==
--festivus
Content-Type: message/rfc822

From: "Cosmo Kramer" <kramer@kramerica.com>
Subject: Exporting my book about coffee tables
Content-Type: multipart/mixed; boundary="giddyup";

--giddyup
Content-Type: text/plain; charset="utf-16"
Content-Transfer-Encoding: quoted-printable

=FF=FE=0C!5=D8"=DD5=D8)=DD5=D8-=DD =005=D8*=DD5=D8"=DD =005=D8"=
=DD5=D85=DD5=D8-=DD5=D8,=DD5=D8/=DD5=D81=DD =005=D8*=DD5=D86=DD =
=005=D8=1F=DD5=D8,=DD5=D8,=DD5=D8(=DD =005=D8-=DD5=D8)=DD5=D8"=
=DD5=D8=1E=DD5=D80=DD5=D8"=DD!=00
--giddyup
Content-Type: image/gif; name*1="about "; name*0="Book ";
              name*2*=utf-8''%e2%98%95 tables.gif
Content-Transfer-Encoding: Base64
Content-Disposition: attachment

R0lGODlhAQABAIAAAAAAAP///yH5BAEAAAAALAAAAAABAAEAAAIBRAA7
--giddyup--
--festivus--
"#
    .as_slice();

    #[test]
    #[expect(clippy::unwrap_used, clippy::non_ascii_literal, reason = "test")]
    fn parse_email() {
        let email = Email::try_from((1, EMAIL_EXAMPLES)).unwrap();
        let headers = email.as_headers();

        assert_eq!(
            headers
                .get(&HeaderName::From)
                .unwrap()
                .as_address()
                .unwrap()
                .first()
                .unwrap(),
            &Addr::new("Art Vandelay (Vandelay Industries)".into(), "art@vandelay.com")
        );

        assert_eq!(
            headers
                .get(&HeaderName::To)
                .unwrap()
                .as_address()
                .unwrap()
                .as_group()
                .unwrap(),
            &[
                Group::new(
                    "Colleagues",
                    vec![Addr::new("James Smythe".into(), "james@vandelay.com")]
                ),
                Group::new(
                    "Friends",
                    vec![
                        Addr::new(None, "jane@example.com"),
                        Addr::new("John Smîth".into(), "john@example.com"),
                    ]
                )
            ]
        );

        assert_eq!(
            headers
                .get(&HeaderName::Date)
                .unwrap()
                .as_datetime()
                .unwrap()
                .to_rfc3339(),
            "2021-11-20T14:22:01-08:00"
        );

        // RFC2047 support for encoded text in message readers
        assert_eq!(
            headers
                .get(&HeaderName::Subject)
                .unwrap()
                .as_text()
                .unwrap(),
            "Why not both importing AND exporting? ☺"
        );

        // HTML and text body parts are returned conforming to RFC8621, Section 4.1.4
        assert_eq!(
            &email.html.unwrap(),
            "<html><p>I was thinking about quitting the &ldquo;exporting&rdquo; to focus just on the &ldquo;importing&rdquo;,</p><p>but then I thought, why not do both? &#x263A;</p></html>"
        );

        // HTML parts are converted to plain text (and vice-versa) when missing
        assert_eq!(
            &email.text.unwrap(),
            "I was thinking about quitting the “exporting” to focus just on the “importing”,\nbut then I thought, why not do both? ☺\n"
        );
    }
}
