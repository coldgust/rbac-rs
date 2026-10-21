-- KEYS[1] = rbac:rt:{fid}:meta
-- ARGV[1] = now (seconds)
-- ARGV[2] = reuse_grace_secs
-- ARGV[3] = refresh_expiration_secs
-- ARGV[4] = presented_th (hash of secret submitted by client)
-- ARGV[5] = new_th (hash of the newly generated secret)
-- ARGV[6] = user_agent
-- ARGV[7] = ip
--
-- Returns one of:
--   {'ok', uid, generation, new_exp}
--   {'retry'}
--   {'reused', uid}
--   {'revoked'}
--   {'expired'}
--   {'invalid'}

local m = redis.call('HMGET', KEYS[1],
  'uid', 'gen', 'cur_th', 'prev_th', 'prev_at', 'abs_exp', 'revoked', 'exp')
if not m[1] then
  return {'invalid'}
end
if m[7] == '1' then
  return {'revoked'}
end

local now = tonumber(ARGV[1])
local abs_exp = tonumber(m[6])
if now >= abs_exp then
  return {'expired'}
end

-- The current refresh token has its own sliding expiry.
local cur_exp = tonumber(m[8]) or 0
if now >= cur_exp then
  return {'expired'}
end

local presented = ARGV[4]

if presented == m[3] then
  -- Current token: normal rotation.
elseif presented == m[4] then
  -- The token that was just rotated away is being used again. Inside the
  -- grace window this is treated as a concurrent refresh; outside it is reuse.
  local prev_at = tonumber(m[5]) or 0
  if now - prev_at <= tonumber(ARGV[2]) then
    return {'retry'}
  end
  redis.call('HSET', KEYS[1], 'revoked', '1')
  redis.call('EXPIRE', KEYS[1], 3600)
  return {'reused', m[1]}
else
  return {'invalid'}
end

local new_exp = math.min(now + tonumber(ARGV[3]), abs_exp)
if new_exp - now < 60 then
  return {'expired'}
end

local new_gen = tonumber(m[2]) + 1

redis.call('HSET', KEYS[1],
  'gen', new_gen,
  'prev_th', m[3],
  'prev_at', now,
  'cur_th', ARGV[5],
  'iat', now,
  'exp', new_exp,
  'last_used_at', now,
  'ua', ARGV[6],
  'ip', ARGV[7])
redis.call('EXPIRE', KEYS[1], abs_exp - now)

return {'ok', m[1], tostring(new_gen), tostring(new_exp)}
