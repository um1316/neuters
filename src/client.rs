use ureq::Request;

pub struct Client {
    agent: ureq::Agent,
    headers: Vec<(String, String)>,
}

impl Client {
    pub fn new(agent: ureq::Agent, headers: Vec<(String, String)>) -> Self {
        let mut default_headers = vec![
            ("User-Agent".to_string(), "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/121.0.0.0 Safari/537.36".to_string()),
            ("Accept".to_string(), "application/json, text/plain, */*".to_string()),
            ("Accept-Language".to_string(), "en-US,en;q=0.9".to_string()),
            ("Referer".to_string(), "https://www.reuters.com/".to_string()),
            ("Origin".to_string(), "https://www.reuters.com".to_string()),
            ("sec-ch-ua".to_string(), r#""Not A(Brand";v="99", "Google Chrome";v="121", "Chromium";v="121""#.to_string()),
            ("sec-ch-ua-mobile".to_string(), "?0".to_string()),
            ("sec-ch-ua-platform".to_string(), r#""Windows""#.to_string()),
            ("Sec-Fetch-Dest".to_string(), "empty".to_string()),
            ("Sec-Fetch-Mode".to_string(), "cors".to_string()),
            ("Sec-Fetch-Site".to_string(), "same-origin".to_string()),
        ];
        
        default_headers.extend(headers);
        
        Self { 
            agent, 
            headers: default_headers 
        }
    }

    pub fn get(&self, path: &str) -> Request {
        let mut request = self.agent.get(path);
        for (key, value) in &self.headers {
            request = request.set(key, value)
        }
        request
    }
}
