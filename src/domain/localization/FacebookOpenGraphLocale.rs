// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[serde(deny_unknown_fields)]
#[derive(Serialize, Deserialize, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(crate) enum FacebookOpenGraphLocale
{
	en_US,
	ca_ES,
	cs_CZ,
	cx_PH,
	cy_GB,
	da_DK,
	de_DE,
	eu_ES,
	en_UD,
	es_LA,
	es_ES,
	gn_PY,
	fi_FI,
	fr_FR,
	gl_ES,
	hu_HU,
	it_IT,
	ja_JP,
	ko_KR,
	nb_NO,
	nn_NO,
	nl_NL,
	fy_NL,
	pl_PL,
	pt_BR,
	pt_PT,
	ro_RO,
	ru_RU,
	sk_SK,
	sl_SI,
	sv_SE,
	th_TH,
	tr_TR,
	ku_TR,
	zh_CN,
	zh_HK,
	zh_TW,
	af_ZA,
	sq_AL,
	hy_AM,
	az_AZ,
	be_BY,
	bn_IN,
	bs_BA,
	bg_BG,
	hr_HR,
	nl_BE,
	en_GB,
	et_EE,
	fo_FO,
	fr_CA,
	ka_GE,
	el_GR,
	gu_IN,
	hi_IN,
	is_IS,
	id_ID,
	ga_IE,
	jv_ID,
	kn_IN,
	kk_KZ,
	lv_LV,
	lt_LT,
	mk_MK,
	mg_MG,
	ms_MY,
	mt_MT,
	mr_IN,
	mn_MN,
	ne_NP,
	pa_IN,
	sr_RS,
	so_SO,
	sw_KE,
	tl_PH,
	ta_IN,
	te_IN,
	ml_IN,
	uk_UA,
	uz_UZ,
	vi_VN,
	km_KH,
	tg_TJ,
	ar_AR,
	he_IL,
	ur_PK,
	fa_IR,
	ps_AF,
	my_MM,
	qz_MM,
	or_IN,
	si_LK,
	rw_RW,
	cb_IQ,
	ha_NG,
	ja_KS,
	br_FR,
	tz_MA,
	co_FR,
	as_IN,
	ff_NG,
	sc_IT,
	sz_PL,
}

impl Default for FacebookOpenGraphLocale
{
	#[inline(always)]
	fn default() -> Self
	{
		FacebookOpenGraphLocale::en_US
	}
}

impl FacebookOpenGraphLocale
{
	#[inline(always)]
	pub(crate) fn to_str(&self) -> &'static str
	{
		use self::FacebookOpenGraphLocale::*;
		
		match *self
		{
			en_US => "en_US",
			ca_ES => "ca_ES",
			cs_CZ => "cs_CZ",
			cx_PH => "cx_PH",
			cy_GB => "cy_GB",
			da_DK => "da_DK",
			de_DE => "de_DE",
			eu_ES => "eu_ES",
			en_UD => "en_UD",
			es_LA => "es_LA",
			es_ES => "es_ES",
			gn_PY => "gn_PY",
			fi_FI => "fi_FI",
			fr_FR => "fr_FR",
			gl_ES => "gl_ES",
			hu_HU => "hu_HU",
			it_IT => "it_IT",
			ja_JP => "ja_JP",
			ko_KR => "ko_KR",
			nb_NO => "nb_NO",
			nn_NO => "nn_NO",
			nl_NL => "nl_NL",
			fy_NL => "fy_NL",
			pl_PL => "pl_PL",
			pt_BR => "pt_BR",
			pt_PT => "pt_PT",
			ro_RO => "ro_RO",
			ru_RU => "ru_RU",
			sk_SK => "sk_SK",
			sl_SI => "sl_SI",
			sv_SE => "sv_SE",
			th_TH => "th_TH",
			tr_TR => "tr_TR",
			ku_TR => "ku_TR",
			zh_CN => "zh_CN",
			zh_HK => "zh_HK",
			zh_TW => "zh_TW",
			af_ZA => "af_ZA",
			sq_AL => "sq_AL",
			hy_AM => "hy_AM",
			az_AZ => "az_AZ",
			be_BY => "be_BY",
			bn_IN => "bn_IN",
			bs_BA => "bs_BA",
			bg_BG => "bg_BG",
			hr_HR => "hr_HR",
			nl_BE => "nl_BE",
			en_GB => "en_GB",
			et_EE => "et_EE",
			fo_FO => "fo_FO",
			fr_CA => "fr_CA",
			ka_GE => "ka_GE",
			el_GR => "el_GR",
			gu_IN => "gu_IN",
			hi_IN => "hi_IN",
			is_IS => "is_IS",
			id_ID => "id_ID",
			ga_IE => "ga_IE",
			jv_ID => "jv_ID",
			kn_IN => "kn_IN",
			kk_KZ => "kk_KZ",
			lv_LV => "lv_LV",
			lt_LT => "lt_LT",
			mk_MK => "mk_MK",
			mg_MG => "mg_MG",
			ms_MY => "ms_MY",
			mt_MT => "mt_MT",
			mr_IN => "mr_IN",
			mn_MN => "mn_MN",
			ne_NP => "ne_NP",
			pa_IN => "pa_IN",
			sr_RS => "sr_RS",
			so_SO => "so_SO",
			sw_KE => "sw_KE",
			tl_PH => "tl_PH",
			ta_IN => "ta_IN",
			te_IN => "te_IN",
			ml_IN => "ml_IN",
			uk_UA => "uk_UA",
			uz_UZ => "uz_UZ",
			vi_VN => "vi_VN",
			km_KH => "km_KH",
			tg_TJ => "tg_TJ",
			ar_AR => "ar_AR",
			he_IL => "he_IL",
			ur_PK => "ur_PK",
			fa_IR => "fa_IR",
			ps_AF => "ps_AF",
			my_MM => "my_MM",
			qz_MM => "qz_MM",
			or_IN => "or_IN",
			si_LK => "si_LK",
			rw_RW => "rw_RW",
			cb_IQ => "cb_IQ",
			ha_NG => "ha_NG",
			ja_KS => "ja_KS",
			br_FR => "br_FR",
			tz_MA => "tz_MA",
			co_FR => "co_FR",
			as_IN => "as_IN",
			ff_NG => "ff_NG",
			sc_IT => "sc_IT",
			sz_PL => "sz_PL",
		}
	}
}
