use anchor_lang::prelude::*;

// This is your program's public key and it will update
// automatically when you build the project.
declare_id!("3Z7cyEBhYf2RUc3oXvD5fCmCTP76x6DToxw3fr3i7wEv");

#[program]
mod hello_anchor {
    use super::*;
    //pub fn initialize(ctx: Context<Initialize>, data: u64) -> Result<()> {
        //ctx.accounts.new_account.data = data;
        //msg!("Changed data to: {}!", data); // Message will show up in the tx logs
        //Ok(())
    //}
    pub fn create_crop_info(ctx: Context<CreateCropInfo>, month: u8, year: u16) -> Result<()> {
        //, day: u8, month: u8, weather: String
        //ctx.accounts.new_account.data = data;
        //msg!("Changed data to: {}!", data); // Message will show up in the tx logs
        //let cert_account = &mut ctx.accounts.cert_account;
        let crop_info = &mut ctx.accounts.crop_info;
        //if !id_no.to_string().as_bytes().len() == 8 {
            // proper error handling omitted for brevity
            //panic!();
        //}
        //TESTS ONLY
        
        //let day: u8 = 1;
        //let weather: String = String::from("norain");
        //let month: u8 = 11;
    
        let valid_month = {
          if month >= 1 && month <= 12 {
              true
          }
          else{false}
        };
        if !valid_month {
            // proper error handling omitted for brevity
            panic!();
        }

        let valid_year = {
          if year >= 2022 && year <= 2023 {
              true
          }
          else{false}
        };
        if !valid_year {
            // proper error handling omitted for brevity
            panic!();
        }
        
        let weather_info = WeatherInfo {
          day: 0,
          weather: WeatherType::Undefined,
        };
        let mut array_weather = [WeatherInfo { day: 0, weather: WeatherType::Undefined }; 3];
        array_weather[0] = weather_info;
        
        let my_month =
        match month {
            1 => Month::January,
            2 => Month::February,
            3 => Month::March,
            4 => Month::April,
            5 => Month::May,
            6 => Month::June,
            7 => Month::July,
            8 => Month::August,
            9 => Month::September,
            10 => Month::October,
            11 => Month::November,
            12 => Month::December,
            _ => Month::Undefined,
        };
        
        //let my_month = get_month(month);
        crop_info.count = 0;
        crop_info.month = my_month;
        crop_info.year = year;
        crop_info.weather = array_weather;

        crop_info.bump = *ctx.bumps.get("crop_info").unwrap();
        Ok(())
    }

    // handler function (add this next to the create_user_stats function in the game module)
    pub fn add_crop_info(ctx: Context<AddCropInfo>, day: u8, month: u8, year: u16, weather: String) -> Result<()> {
        //, day: u8, month: u8, weather: String
        //TESTS ONLY
        
        //let day: u8 = 1;
        //let weather: String = String::from("plentyrain");
        //let month: u8 = 11;
        let valid_day = {
          if day >= 1 && day <= 31 {
              true
          }
            else{false}
        };
        if !valid_day {
            // proper error handling omitted for brevity
            panic!();
        }
    
        let valid_month = {
          if month >= 1 && month <= 12 {
              true
          }
          else{false}
        };
        if !valid_month {
            // proper error handling omitted for brevity
            panic!();
        }

        let valid_year = {
          if year >= 2022 && year <= 2023 {
              true
          }
          else{false}
        };
        if !valid_year {
            // proper error handling omitted for brevity
            panic!();
        }
        
        if weather.as_bytes().len() > 15 {
            // proper error handling omitted for brevity
            panic!();
        }

        let is_weather =
        match weather.to_lowercase().as_str() {
            "norain" => true,
            "poorrain" => true,
            "plentyrain" => true,
            _ => false,
        };
        if !is_weather {
            // proper error handling omitted for brevity
            panic!();
        }
        
        let my_weather =
        match weather.to_lowercase().as_str() {
            "norain" => WeatherType::NoRain,
            "poorrain" => WeatherType::PoorRain,
            "plentyrain" => WeatherType::PlentyRain,
            _ => WeatherType::Undefined,
        };
        let weather_info = WeatherInfo {
          day: day,
          weather: my_weather,
        };
        
        let my_month =
        match month {
            1 => Month::January,
            2 => Month::February,
            3 => Month::March,
            4 => Month::April,
            5 => Month::May,
            6 => Month::June,
            7 => Month::July,
            8 => Month::August,
            9 => Month::September,
            10 => Month::October,
            11 => Month::November,
            12 => Month::December,
            _ => Month::Undefined,
        };
        
        //let my_month = get_month(month);
        let x: usize = usize::from(day) - 1;

        let is_plenty_rain =
        match my_weather {
            WeatherType::PlentyRain => true,
            _ => false,
        };

        //lets update the counter when there is "norain"/"poorrain"
        if !is_plenty_rain {
            ctx.accounts.crop_info.count += 1;
        }
        
        //ctx.accounts.crop_info.count = 1;
        ctx.accounts.crop_info.month = my_month;
        ctx.accounts.crop_info.year = year;
        ctx.accounts.crop_info.weather[x] = weather_info;
        /*
        emit!(MyEvent {
            data: 5,
            label: [1,2,3,4,5],
        });
        */
        /*
        emit!(MyEvent {
            data: true,
            label: String::from("status"),
        });
        */
        
        //lets send notify status as true when count is reached for "norain"/"poorrain"
        let is_notify = {
            if ctx.accounts.crop_info.count >= 5 {
                true
            }
            else{false}    
        };

        emit!(MyEvent {
            data: is_notify,
            label: String::from("status"),
        });
        
        Ok(())
    }
    //Consider creating another module and add this function
    //then call this function in this module
    /*
    pub fn get_month(month: u8) -> Month {
        let my_month = match month {
            1 => Month::January,
            2 => Month::February,
            3 => Month::March,
            4 => Month::April,
            5 => Month::May,
            6 => Month::June,
            7 => Month::July,
            8 => Month::August,
            9 => Month::September,
            10 => Month::October,
            11 => Month::November,
            12 => Month::December,
            _ => Month::Undefined,
        };
        my_month
    }
    */
}

#[derive(Accounts)]
pub struct CreateCropInfo<'info> {
    // We must specify the space in order to initialize an account.
    // First 8 bytes are default account discriminator,
    // next 8 bytes come from CertAccount.data being type u64.
    // (u64 = 64 bits unsigned integer = 8 bytes)
    //#[account(init, payer = signer, space = 8 + 8)]
    //space: 8 discriminator + 2 id_no + 4 name length + 200 name + 1 bump
    //space = 8 + 2 + 4 + 40 + 1, seeds = [b"cert-vault", signer.key().as_ref()], bump
    //space = 8 + 1 + 1 + 1 + (2*3)), seeds = [b"crop-info", signer.key().as_ref()], bump 
    #[account(
        init,
        payer = signer,
        space = 20, seeds = [b"crop-info", signer.key().as_ref()], bump
    )]
    pub crop_info: Account<'info, CropInfo>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// validation struct
#[derive(Accounts)]
pub struct AddCropInfo<'info> {
    pub signer: Signer<'info>,
    #[account(mut, seeds = [b"crop-info", signer.key().as_ref()], bump = crop_info.bump)]
    pub crop_info: Account<'info, CropInfo>,
}
/*
#[account]
pub struct CertVault {
    day: u8,
    month: u8,
    weather: String,
    bump: u8,
}
*/
//#[allow(dead_code)]
//#[derive(Debug, Copy, Clone)]
//#[account]

#[event]
pub struct MyEvent {
    pub data: bool,
    pub label: String,
    //pub data: u64,
    //pub label: [u8; 5],
}

#[derive(AnchorSerialize, AnchorDeserialize, Copy, Clone)]
pub enum Month {
    January,
    February,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
    December,
    Undefined,
}

//#[allow(dead_code)]
//#[derive(Debug, Copy, Clone)]
//#[account]
//#[derive(AnchorSerialize, AnchorDeserialize, Clone)]

#[derive(AnchorSerialize, AnchorDeserialize, Copy, Clone)]
pub enum WeatherType {
    NoRain,
    PoorRain,
    PlentyRain,
    Undefined,
}

impl Default for WeatherType {
    fn default() -> Self { WeatherType::Undefined }
}

//#[allow(dead_code)]
//#[derive(Debug, Copy, Clone)]
//#[account]

#[derive(AnchorSerialize, AnchorDeserialize, Default, Copy, Clone)]
pub struct WeatherInfo {
  day: u8,
  weather: WeatherType,
}

//impl Copy for WeatherInfo {}
/*
impl Clone for WeatherInfo {
    fn clone(&self) -> WeatherInfo {
        WeatherInfo{day: self.day, weather: self.weather}
    }
}
*/
/*
impl Default for WeatherInfo {
    fn default() -> Self {
        Self {
            day: 0,
            weather: WeatherType::Undefined,
        }
    }
}
*/
//#[allow(dead_code)]
//#[derive(Debug, Copy, Clone)]
#[account]
pub struct CropInfo {
  count: u8,
  month: Month,
  year: u16,
  weather: [WeatherInfo; 3],
  bump: u8,
}