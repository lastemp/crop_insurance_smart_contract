use anchor_lang::prelude::*;

// This is your program's public key and it will update
// automatically when you build the project.
declare_id!("11111111111111111111111111111111");

#[program]
mod hello_anchor {
    use super::*;

    pub fn create_crop_info(ctx: Context<CreateCropInfo>, month: u8, year: u16) -> Result<()> {

        let crop_info = &mut ctx.accounts.crop_info;
    
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

    // handler function
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
        
        ctx.accounts.crop_info.month = my_month;
        ctx.accounts.crop_info.year = year;
        ctx.accounts.crop_info.weather[x] = weather_info;
        
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
    //#[account(init, payer = signer, space = )]
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

#[event]
pub struct MyEvent {
    pub data: bool,
    pub label: String,
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

#[derive(AnchorSerialize, AnchorDeserialize, Default, Copy, Clone)]
pub struct WeatherInfo {
  day: u8,
  weather: WeatherType,
}

#[account]
pub struct CropInfo {
  count: u8,
  month: Month,
  year: u16,
  weather: [WeatherInfo; 3],
  bump: u8,
}