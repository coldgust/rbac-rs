import { createI18n } from 'vue-i18n'
import en from './en/index'
import zh from './zh/index'

const savedLocale = localStorage.getItem('locale') || 'zh'

const i18n = createI18n({
  legacy: false,
  locale: savedLocale,
  fallbackLocale: 'en',
  messages: { en, zh },
})

export default i18n
